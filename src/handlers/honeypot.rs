use crate::ButlerResult;
use crate::db::action_journal::ModerationAction;
use crate::handlers::{Handler, MSG_CACHE};
use color_eyre::eyre::{Context as _, ContextCompat};
use serenity::all::{ChannelId, GetMessages, MessageId, UserId};
use serenity::all::{Context, CreateEmbed, Message};
use std::ops::Not;
use time::OffsetDateTime;
use tracing::{info, warn};

impl Handler {
    #[tracing::instrument(skip(self, ctx, msg), err)]
    pub async fn handle_honeypot(&self, ctx: Context, msg: &Message) -> ButlerResult<()> {
        let Some(guild_id) = msg.guild_id else {
            return Ok(());
        };

        let honeypot = self
            .database
            .get_honeypot_from_guild_id(guild_id)
            .await
            .with_context(|| format!("Failed to fetch honeypot for guild {}", guild_id))?;

        let Some(honeypot) = honeypot else {
            return Ok(());
        };

        if honeypot.enabled.not() {
            return Ok(());
        }

        if !honeypot
            .channel_ids
            .contains(&(msg.channel_id.get() as i64))
        {
            return Ok(());
        }

        // We are in an armed honeypot channel. Get roles, falling back to HTTP if missing.
        let roles = match msg.member.as_ref() {
            Some(m) => m.roles.clone(),
            None => {
                match guild_id.member(&ctx.http, msg.author.id).await {
                    Ok(member) => member.roles.clone(),
                    Err(_) => return Ok(()), // User left or is a webhook
                }
            }
        };

        // Ignore whitelisted roles
        if honeypot
            .safe_role_ids
            .iter()
            .any(|&safe| roles.iter().any(|role| safe == role.get() as i64))
        {
            info!(
                "{} talked in {} but their role is whitelisted",
                msg.author.name,
                msg.channel_id.name(&ctx).await?
            );
            return Ok(());
        }

        let posted = OffsetDateTime::from_unix_timestamp(msg.timestamp.unix_timestamp())?;
        let now = OffsetDateTime::now_local()?;
        let visible_ms = (now - posted).whole_milliseconds();

        let reason = format!(
            "Kicked {} for sending message into {}\nVisible for {}ms before kick",
            msg.author.name,
            msg.channel(&ctx).await?,
            visible_ms
        );

        info!("Attempting to kick {} from honeypot...", msg.author.name);
        if let Err(e) = guild_id
            .kick_with_reason(ctx.clone(), msg.author.id, &reason)
            .await
        {
            tracing::error!(
                "Failed to kick {}. Check permissions and role ordering. Error: {}",
                msg.author.name,
                e
            );
            return Err(e.into());
        }
        warn!(
            "Successfully kicked {} for sending message into {} (visible: {}ms)",
            msg.author.name,
            msg.channel_id.name(&ctx).await?,
            visible_ms
        );
        self.database
            .log_action_to_journal(
                guild_id,
                msg.author.id,
                ModerationAction::KickedHoneypot,
                None,
            )
            .await?;

        info!("Started cleaning up after {}", msg.author.id);
        let (fast, scan) = self.cleanup_last_hour(&ctx, msg).await?;
        let total = fast + scan;

        let cleanup_time = OffsetDateTime::now_local()?;
        let cleanup_dur =
            std::time::Duration::from_millis((cleanup_time - posted).whole_milliseconds() as u64);
        let embed = CreateEmbed::new()
            .title("Honeypot Kick")
            .color(0xED4245)
            .field("User", msg.author.to_string(), true)
            .field("Channel", msg.channel(&ctx).await?.to_string(), true)
            .field("Visible", format!("{}ms", visible_ms), true)
            .field(
                "Deleted",
                format!("{} cache / {} scan / {} total", fast, scan, total),
                false,
            )
            .footer(serenity::all::CreateEmbedFooter::new(format!(
                "Cleanup took {}",
                humantime::format_duration(cleanup_dur)
            )));
        self.log_embed(&ctx, embed, guild_id).await?;

        Ok(())
    }

    /// Deletes all messages of user for past hour
    /// Returns (fast_pass_count, scan_pass_count)
    #[tracing::instrument(skip(self, ctx, msg), err)]
    pub async fn cleanup_last_hour(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> ButlerResult<(u64, u64)> {
        let guild_id = msg.guild_id.context("missing guild id")?;

        let user_id = msg.author.id;

        // Get all channels in the guild
        let channels = guild_id
            .channels(&ctx.http)
            .await
            .with_context(|| format!("Failed to fetch channels for guild {}", guild_id))?;

        // Fastpass deleting known cached messages
        let cached: Vec<(ChannelId, Vec<MessageId>)> = MSG_CACHE
            .iter()
            .filter(|entry| entry.key().0 == guild_id && entry.key().1 == user_id)
            .map(|entry| (entry.key().2, entry.value().clone()))
            .collect();
        let fast_count = cached.iter().map(|(_, msgs)| msgs.len() as u64).sum();
        for (key, _) in &cached {
            MSG_CACHE.remove(&(guild_id, user_id, *key));
        }
        for (channel, messages) in cached {
            for chunk in messages.chunks(100) {
                if chunk.len() == 1 {
                    if let Err(e) = channel.delete_message(&ctx.http, chunk[0]).await {
                        tracing::warn!("Failed to delete single message in fastpass: {}", e);
                    }
                } else if !chunk.is_empty() {
                    if let Err(e) = channel.delete_messages(&ctx.http, chunk.to_vec()).await {
                        tracing::warn!("Failed to bulk delete messages in fastpass: {}", e);
                    }
                }
            }
        }

        // Slow pass
        let mut scan_count = 0u64;
        for (channel_id, channel) in channels {
            if channel.is_text_based() {
                // Scan up to 300 messages per channel
                let mut last_id = None;
                for _ in 0..3 {
                    last_id = self
                        .clean_channel_after(ctx, channel_id, user_id, last_id, &mut scan_count)
                        .await?;
                    if last_id.is_none() {
                        break;
                    }
                }
            }
        }
        Ok((fast_count, scan_count))
    }

    #[tracing::instrument(skip(self, ctx), err)]
    async fn clean_channel_after(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        user_id: UserId,
        before_id: Option<MessageId>,
        count: &mut u64,
    ) -> ButlerResult<Option<MessageId>> {
        let mut req = GetMessages::new().limit(100);
        if let Some(id) = before_id {
            req = req.before(id);
        }

        match channel_id.messages(&ctx.http, req).await {
            Ok(messages) => {
                let mut last = None;
                for message in messages {
                    if message.author.id == user_id {
                        if let Err(e) = channel_id.delete_message(&ctx.http, message.id).await {
                            tracing::warn!("Failed to delete message in scan pass: {}", e);
                        } else {
                            *count += 1;
                        }
                    }
                    last = Some(message.id);
                }
                Ok(last)
            }
            Err(e) => {
                tracing::warn!("Failed to fetch messages for channel {}: {}", channel_id, e);
                Ok(None)
            }
        }
    }
}

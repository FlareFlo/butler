use crate::ButlerResult;
use crate::db::action_journal::ModerationAction;
use crate::handlers::{Handler, MSG_CACHE};
use color_eyre::eyre::{Context as EyreContext, ContextCompat};
use serenity::all::{ChannelId, GetMessages, MessageId, UserId};
use serenity::all::{Context, Message};
use std::ops::Not;
use time::OffsetDateTime;
use tracing::{info, warn};

impl Handler {
    pub async fn handle_honeypot(&self, ctx: Context, msg: &Message) -> ButlerResult<()> {
        if let Ok(member) = msg.member(ctx.clone()).await {
            let honeypot = self
                .database
                .get_honeypot_from_guild_id(member.guild_id)
                .await?;

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
            // Ignore whitelisted roles
            if honeypot
                .safe_role_ids
                .iter()
                .any(|&safe| member.roles.iter().any(|role| safe == role.get() as i64))
            {
                info!(
                    "{} talked in {} but their role is whitelisted",
                    member.display_name(),
                    msg.channel_id.name(&ctx).await?
                );
                return Ok(());
            }

            let posted = OffsetDateTime::from_unix_timestamp(msg.timestamp.unix_timestamp())?;
            let now = OffsetDateTime::now_local()?;
            let visible_ms = (now - posted).whole_milliseconds();

            let reason = format!(
                "Kicked {} for sending message into {}\nVisible for {}ms before kick",
                member,
                msg.channel(&ctx).await?,
                visible_ms
            );

            member
                .kick_with_reason(ctx.clone(), &reason)
                .await
                .with_context(|| format!("Failed to kick {}", member.display_name()))?;
            warn!(
                "Kicked {} for sending message into {} (visible: {}ms)",
                member.display_name(),
                msg.channel_id.name(&ctx).await?,
                visible_ms
            );
            self.database
                .log_action_to_journal(
                    member.guild_id,
                    member.user.id,
                    ModerationAction::KickedHoneypot,
                    None,
                )
                .await?;

            info!("Started cleaning up after {}", member.user.id);
            let (fast, scan) = self.cleanup_last_hour(&ctx, msg).await?;
            let total = fast + scan;

            let cleanup_time = OffsetDateTime::now_local()?;
            let cleanup_dur = std::time::Duration::from_millis((cleanup_time - posted).whole_milliseconds() as u64);
            let log_msg = format!(
                "{}\nCleaned up after {} (cache: {}, scan: {}, total: {})",
                reason, humantime::format_duration(cleanup_dur), fast, scan, total
            );
            self.log_discord(&ctx, &log_msg, member.guild_id).await?;
        }
        Ok(())
    }

    /// Deletes all messages of user for past hour
    /// Returns (fast_pass_count, scan_pass_count)
    pub async fn cleanup_last_hour(&self, ctx: &Context, msg: &Message) -> ButlerResult<(u64, u64)> {
        let guild_id = msg.guild_id.context("missing guild id")?;

        let user_id = msg.author.id;

        // Get all channels in the guild
        let channels = guild_id.channels(&ctx.http).await?;

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
                channel.delete_messages(ctx.http.clone(), chunk.to_vec()).await?;
            }
        }

        // Slow pass
        let mut scan_count = 0u64;
        for (channel_id, channel) in channels {
            if channel.is_text_based() {
                // Scan up to 300 messages per channel
                let mut last_id = Some(msg.id);
                for _ in 0..3 {
                    match last_id {
                        Some(mid) => {
                            last_id = self.clean_channel_after(ctx, channel_id, user_id, mid, &mut scan_count).await?;
                            if last_id == Some(mid) {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
        Ok((fast_count, scan_count))
    }

    async fn clean_channel_after(&self, ctx: &Context, channel_id: ChannelId, user_id: UserId, message_id: MessageId, count: &mut u64) -> ButlerResult<Option<MessageId>> {
        let mut last_id = None;
        // Fetch up to 100 most recent messages (API limit)
        if let Ok(messages) = channel_id
            .messages(&ctx.http, GetMessages::new().limit(100).before(message_id))
            .await
        {
            for message in messages {
                if message.author.id == user_id
                {
                    channel_id.delete_message(&ctx.http, message.id).await?;
                    *count += 1;
                }
                last_id = Some(message.id);
            }
        }
        Ok(last_id)
    }
}

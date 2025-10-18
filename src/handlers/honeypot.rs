use crate::ButlerResult;
use crate::handlers::Handler;
use color_eyre::eyre::ContextCompat;
use serenity::all::GetMessages;
use serenity::all::{Context, Message};
use std::ops::Not;
use time::{Duration, OffsetDateTime};
use tracing::{error, info, warn};

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

            let reason = format!(
                "Kicked {} for sending message into {}",
                member,
                msg.channel(&ctx).await?
            );

            if let Err(err) = member.kick_with_reason(ctx.clone(), &reason).await {
                error!("Failed to ban {}: {:?}", member.display_name(), err);
            } else {
                warn!(
                    "Kicked {} for sending message into {}",
                    member.display_name(),
                    msg.channel_id.name(&ctx).await?
                );
                self.log_discord(&ctx, &reason, member.guild_id).await?;
            }

            self.cleanup_last_hour(&ctx, msg).await?;
        }
        Ok(())
    }

    /// Deletes all messages of user for past hour
    pub async fn cleanup_last_hour(&self, ctx: &Context, msg: &Message) -> ButlerResult<()> {
        let guild_id = msg.guild_id.context("missing guild id")?;

        let user_id = msg.author.id;
        let one_hour_ago = OffsetDateTime::now_local()? - Duration::hours(1);

        // Get all channels in the guild
        let channels = guild_id.channels(&ctx.http).await?;

        for (channel_id, channel) in channels {
            if channel.is_text_based() {
                // Fetch up to 100 most recent messages (API limit)
                if let Ok(messages) = channel_id
                    .messages(&ctx.http, GetMessages::new().limit(100))
                    .await
                {
                    for message in messages {
                        if message.author.id == user_id
                            && message.timestamp.unix_timestamp() > one_hour_ago.unix_timestamp()
                        {
                            channel_id.delete_message(&ctx.http, message.id).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

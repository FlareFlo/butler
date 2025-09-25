use serenity::all::{ChannelType, GetMessages, GuildChannel};
use serenity::all::Channel;
use serenity::all::{Context, Message};
use time::{Duration, UtcDateTime};
use tracing::{error, warn};
use crate::{util, CONFIG};

pub async fn handle_honeypot(ctx: Context, msg: &Message) {
    if let Ok(member) = msg.member(ctx.clone()).await
        && CONFIG.honeypot_channels.contains(&msg.channel_id.get())
    {
        // Ignore whitelisted roles
        if CONFIG
            .honeypot_safe_roles
            .iter()
            .any(|&safe| member.roles.iter().any(|role| safe == role.get()))
        {
            return;
        }

        let reason = format!( "Kicked {} for sending message into honeypot {}",
                              member.user.name,
                              msg.channel(&ctx)
                                  .await
                                  .unwrap()
                                  .guild()
                                  .expect("user to be a member of this guild")
                                  .name);

        if let Err(err) = member
            .kick_with_reason(ctx.clone(), &reason)
            .await
        {
            error!("Failed to ban {}: {:?}", member.user.name, err);
        } else {
            warn!("{reason}");
            util::log_discord(
                &ctx,
                &reason,
            )
            .await;
        }

        cleanup_last_hour(&ctx, msg).await;
    }
}

/// Deletes all messages of user for past hour
pub async fn cleanup_last_hour(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id.expect("we know its a guild");

    let user_id = msg.author.id;
    let one_hour_ago = UtcDateTime::now() - Duration::hours(1);

    // Get all channels in the guild
    let channels = guild_id.channels(&ctx.http).await.unwrap();

    for (channel_id, channel) in channels {
        if channel.is_text_based() {
            // Fetch up to 100 most recent messages (API limit)
            if let Ok(messages) = channel_id.messages(&ctx.http, GetMessages::new().limit(100)).await {
                for message in messages {
                    if message.author.id == user_id && message.timestamp.unix_timestamp() > one_hour_ago.unix_timestamp() {
                        let _ = channel_id.delete_message(&ctx.http, message.id).await;
                    }
                }
            }
        }
    }
}
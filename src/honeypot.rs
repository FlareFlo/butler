use serenity::all::{Context, Message};
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
    }
}
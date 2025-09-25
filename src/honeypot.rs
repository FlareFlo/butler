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

        if let Err(err) = member
            .ban_with_reason(ctx.clone(), 1, "Banned for using honeypot")
            .await
        {
            error!("Failed to ban {}: {:?}", member.user.name, err);
        } else {
            warn!(
                "Banned {} for sending message into honeypot {}",
                member.user.name,
                msg.channel(&ctx)
                    .await
                    .unwrap()
                    .guild()
                    .expect("user to be a member of this guild")
                    .name
            );
            util::log_discord(
                &ctx,
                &format!("Banned {} for using honeypot", member.user.name),
            )
            .await;
        }
    }
}
use serenity::all::{ChannelId, Context, CreateMessage};
use crate::CONFIG;

pub async fn log_discord(ctx: &Context, reason: &str) {
    let log_message = CreateMessage::new().content(reason);
    if let Err(e) = ChannelId::new(CONFIG.log_chat)
        .send_message(&ctx.http, log_message)
        .await
    {
        dbg!(e);
    };
}
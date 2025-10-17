use crate::Handler;
use serenity::all::{ChannelId, Context, CreateMessage};

impl Handler {
    pub async fn log_discord(&self, ctx: &Context, reason: &str) {
        let log_message = CreateMessage::new().content(reason);
        if let Err(e) = ChannelId::new(self.config.log_chat)
            .send_message(&ctx.http, log_message)
            .await
        {
            dbg!(e);
        };
    }
}

use color_eyre::eyre::ContextCompat;
use crate::ButlerResult;
use serenity::all::{ChannelId, Context, CreateMessage, GuildId};
use sqlx::query;
use crate::handlers::Handler;

impl Handler {
    pub async fn log_discord(&self, ctx: &Context, reason: &str, guild_id: GuildId) -> ButlerResult<()> {
        let query = query!("
SELECT logging_channel
FROM guilds
WHERE id = $1
", guild_id.get() as i64).fetch_optional(&self.database.pool).await?.context("logging channel without message")?;

        let log_message = CreateMessage::new().content(reason);
        ChannelId::new(query.logging_channel.context("")? as _)
            .send_message(&ctx.http, log_message)
            .await?;
        Ok(())
    }
}

use std::fmt::Display;
use crate::ButlerResult;
use crate::handlers::Handler;
use color_eyre::eyre::ContextCompat;
use serenity::all::{ChannelId, Context, CreateMessage, GuildId};
use sqlx::query;

impl Handler {
    pub async fn process_result<T, E: Display>(
        &self,
        ctx: &Context,
        res: Result<T, E>,
        guild_id: Option<GuildId>,
    ) {
        if let Err(error) = res {
            let errstr = error.to_string();
            tracing::error!("{}", errstr);
            if let Some(guild_id) = guild_id {
                let err = self.log_discord(ctx, &errstr, guild_id).await;
                if let Err(err) = err {
                    tracing::error!("Failed to log to discord: {}", err.to_string());
                }
            }
        }
    }

    pub async fn log_discord(
        &self,
        ctx: &Context,
        reason: &str,
        guild_id: GuildId,
    ) -> ButlerResult<()> {
        let query = query!(
            "
SELECT logging_channel
FROM guilds
WHERE id = $1
",
            guild_id.get() as i64
        )
        .fetch_optional(&self.database.pool)
        .await?
        .context("logging channel without message")?;

        let log_message = CreateMessage::new().content(reason);
        ChannelId::new(query.logging_channel.context("")? as _)
            .send_message(&ctx.http, log_message)
            .await?;
        Ok(())
    }
}

use crate::ButlerResult;
use crate::commands::{Data, PoiseContext};
use serenity::all::{ChannelId, GuildId};
use sqlx::query;

impl Data {
    pub async fn set_logging_channel(
        &self,
        ctx: &PoiseContext<'_>,
        channel: ChannelId,
        guild: GuildId,
    ) -> ButlerResult<()> {
        query!(
            "
			UPDATE guilds
			SET logging_channel = $1
			WHERE id = $2
 		",
            channel.get() as i64,
            guild.get() as i64
        )
        .execute(&ctx.data().pool)
        .await?;
        Ok(())
    }

    pub async fn reset_logging_channel(
        &self,
        ctx: &PoiseContext<'_>,
        guild: GuildId,
    ) -> ButlerResult<()> {
        query!(
            "
			UPDATE guilds
			SET logging_channel = NULL
			WHERE id = $1
 		",
            guild.get() as i64
        )
        .execute(&ctx.data().pool)
        .await?;
        Ok(())
    }
}

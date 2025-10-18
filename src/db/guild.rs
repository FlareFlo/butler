use crate::ButlerResult;
use crate::commands::Data;
use serenity::all::GuildId;
use sqlx::query;
use tracing::warn;

impl Data {
    pub async fn delete_guild(&self, guild_id: GuildId) -> ButlerResult<()> {
        query!(
            "
			DELETE FROM guilds
			WHERE id = $1
		",
            guild_id.get() as i64
        )
        .execute(&self.pool)
        .await?;
        warn!("Deleted guild {}", guild_id.get());
        Ok(())
    }
}

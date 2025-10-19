use crate::ButlerResult;
use crate::commands::Data;
use serenity::all::GuildId;
use sqlx::query;

pub mod action_journal;
mod guild;
pub mod honeypot;
pub mod logging_channel;

impl Data {
    pub async fn ensure_guild_exists(&self, guild_id: GuildId) -> ButlerResult<()> {
        query!(
            "
            INSERT INTO guilds (id)
            VALUES ($1)
            ON CONFLICT (id) DO NOTHING
            ",
            guild_id.get() as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

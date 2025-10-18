use crate::ButlerResult;
use crate::commands::Data;
use crate::db::honeypot::Honeypot;
use serenity::all::GuildId;
use sqlx::{query, query_as};

pub mod honeypot;
mod logging_channel;

impl Data {
    pub async fn ensure_guild_exists(&self, guild_id: GuildId) -> ButlerResult<()> {
        query!(
            "
            INSERT INTO guilds (id)
            VALUES ($1)
            ",
            guild_id.get() as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

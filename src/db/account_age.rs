use crate::ButlerResult;
use crate::commands::Data;
use serenity::all::GuildId;
use sqlx::query;
use std::time::Duration;

impl Data {
    pub async fn get_minimum_account_age(
        &self,
        guild_id: GuildId,
    ) -> ButlerResult<Option<Duration>> {
        self.ensure_guild_exists(guild_id).await?;

        let record = query!(
            "
			SELECT account_minimum_age
			FROM guilds
			WHERE id = $1
		",
            guild_id.get() as i64
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record
            .account_minimum_age
            .map(|e| Duration::from_secs(e as u64)))
    }

    pub async fn set_minimum_account_age(
        &self,
        guild_id: GuildId,
        seconds: Option<Duration>,
    ) -> ButlerResult<()> {
        self.ensure_guild_exists(guild_id).await?;

        let record = query!(
            "
			UPDATE guilds
			SET account_minimum_age = $1
			WHERE id = $2
		",
            seconds.map(|e| e.as_secs() as i64),
            guild_id.get() as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

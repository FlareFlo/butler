use crate::ButlerResult;
use crate::commands::Data;
use itertools::Itertools;
use serenity::all::{ChannelId, GuildId, RoleId};
use sqlx::{query, query_as};

#[derive(Debug, sqlx::FromRow)]
pub struct Honeypot {
    #[allow(unused)]
    pub guild_id: i64,
    pub channel_ids: Vec<i64>,
    pub safe_role_ids: Vec<i64>,
    pub enabled: bool,
}

impl Data {
    pub async fn get_honeypot_from_guild_id(
        &self,
        guild_id: GuildId,
    ) -> ButlerResult<Option<Honeypot>> {
        let honeypot = query_as!(
            Honeypot,
            "
            SELECT *
            FROM honeypot
            WHERE guild_id = $1;
            ",
            guild_id.get() as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(honeypot)
    }

    pub async fn set_honeypot_for_guild(
        &self,
        guild_id: GuildId,
        channel_ids: impl Iterator<Item = ChannelId>,
        safe_role_ids: impl Iterator<Item = RoleId>,
        enabled: bool,
    ) -> ButlerResult<()> {
        self.ensure_guild_exists(guild_id).await?;

        query!(
            "
                INSERT INTO honeypot (guild_id, channel_ids, safe_role_ids, enabled)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (guild_id)
                DO UPDATE
                SET channel_ids = $2, safe_role_ids = $3, enabled = $4;
            ",
            guild_id.get() as i64,
            &channel_ids.map(|e| e.get() as i64).collect_vec(),
            &safe_role_ids.map(|e| e.get() as i64).collect_vec(),
            enabled
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

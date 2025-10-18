use crate::ButlerResult;
use crate::commands::Data;
use itertools::Itertools;
use serenity::all::{Channel, ChannelId, GuildId, Role, RoleId};
use sqlx::{query, query_as};

#[derive(sqlx::FromRow)]
pub struct Honeypot {
    #[allow(unused)]
    pub id: i64,
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
            SELECT h.*
            FROM guilds g
            JOIN honeypot h ON g.honeypot = h.id
            WHERE g.id = $1;
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
        query!(
            "
            WITH honeypot_row AS (
                INSERT INTO honeypot (channel_ids, safe_role_ids, enabled)
                VALUES ($2, $3, $4)
                RETURNING id
            )
            UPDATE guilds
            SET honeypot = (SELECT id FROM honeypot_row)
            WHERE id = $1
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

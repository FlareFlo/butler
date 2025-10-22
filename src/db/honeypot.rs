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

    // Returns honeypot on success, fails if it did not exist
    pub async fn add_safe_role(
        &self,
        guild_id: GuildId,
        safe_role_ids: RoleId,
    ) -> ButlerResult<Option<Honeypot>> {
        let Some(mut honeypot) = self.get_honeypot_from_guild_id(guild_id).await? else {
            return Ok(None);
        };

        query!(
            "
                UPDATE honeypot
                SET safe_role_ids = array_append(safe_role_ids, $2)
                WHERE guild_id = $1
                AND NOT ($2 = ANY(safe_role_ids));

            ",
            guild_id.get() as i64,
            safe_role_ids.get() as i64,
        )
        .execute(&self.pool)
        .await?;

        honeypot.safe_role_ids.push(safe_role_ids.get() as i64);

        Ok(Some(honeypot))
    }

    // Returns honeypot on success, fails if it did not exist
    pub async fn remove_safe_role(
        &self,
        guild_id: GuildId,
        safe_role_id: RoleId,
    ) -> ButlerResult<Option<Honeypot>> {
        let Some(mut honeypot) = self.get_honeypot_from_guild_id(guild_id).await? else {
            return Ok(None);
        };

        query!(
            "
                UPDATE honeypot
                SET safe_role_ids = array_remove(safe_role_ids, $2)
                WHERE guild_id = $1;
            ",
            guild_id.get() as i64,
            safe_role_id.get() as i64,
        )
        .execute(&self.pool)
        .await?;

        honeypot.safe_role_ids.retain(|id| *id != safe_role_id.get() as i64);

        Ok(Some(honeypot))
    }


    // Returns honeypot on success, fails if it did not exist
    pub async fn add_honeypot_channel(
        &self,
        guild_id: GuildId,
        channel_id: ChannelId,
    ) -> ButlerResult<Option<Honeypot>> {
        let Some(mut honeypot) = self.get_honeypot_from_guild_id(guild_id).await? else {
            return Ok(None);
        };

        query!(
            "
                UPDATE honeypot
                SET channel_ids = array_append(channel_ids, $2)
                WHERE guild_id = $1
                AND NOT ($2 = ANY(channel_ids));

            ",
            guild_id.get() as i64,
            channel_id.get() as i64,
        )
            .execute(&self.pool)
            .await?;

        honeypot.channel_ids.push(channel_id.get() as i64);

        Ok(Some(honeypot))
    }

    // Returns honeypot on success, fails if it did not exist
    pub async fn remove_honeypot_channel(
        &self,
        guild_id: GuildId,
        channel_id: ChannelId,
    ) -> ButlerResult<Option<Honeypot>> {
        let Some(mut honeypot) = self.get_honeypot_from_guild_id(guild_id).await? else {
            return Ok(None);
        };

        query!(
            "
                UPDATE honeypot
                SET channel_ids = array_remove(channel_ids, $2)
                WHERE guild_id = $1;
            ",
            guild_id.get() as i64,
            channel_id.get() as i64,
        )
            .execute(&self.pool)
            .await?;

        honeypot.channel_ids.retain(|id| *id != channel_id.get() as i64);

        Ok(Some(honeypot))
    }
}

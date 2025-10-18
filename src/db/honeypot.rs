use crate::ButlerResult;
use crate::commands::Data;
use serenity::all::GuildId;
use sqlx::query_as;

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
}

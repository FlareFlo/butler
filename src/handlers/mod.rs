use crate::commands::Data;
use crate::db::honeypot::Honeypot;
use crate::{ButlerResult, Config};
use poise::async_trait;
use serenity::all::{ActivityData, Context, EventHandler, GuildId, Member, Message, Ready};
use sqlx::query_as;
use tracing::info;

mod account_age;
mod honeypot;

pub struct Handler {
    pub database: Data,
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let res = self.check_account_age(&ctx, &new_member).await;
        self.process_result(&ctx, res, Some(new_member.guild_id))
            .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        handle_dm(ctx.clone(), &msg).await;
        let res = self.handle_honeypot(ctx.clone(), &msg).await;
        self.process_result(&ctx, res, msg.guild_id).await;
    }

    async fn ready(&self, cx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        cx.set_activity(Some(ActivityData::watching("for bad actors")))
    }
}

async fn handle_dm(ctx: Context, msg: &Message) {
    // Ignore non-DMs
    if msg.guild_id.is_some() {
        return;
    }
    if ctx.cache.current_user().id == msg.author.id {
        return;
    }
    info!("{} said {}", msg.author.name, msg.content);
}

impl Data {
    async fn get_honeypot_from_guild_id(
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

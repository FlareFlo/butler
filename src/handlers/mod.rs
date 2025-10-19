use crate::Config;
use crate::commands::Data;
use poise::async_trait;
use serenity::all::{
    ActivityData, Context, EventHandler, Guild, Member, Message, Ready, UnavailableGuild,
};
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

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, _full: Option<Guild>) {
        // This means the bot has been removed from the server
        if incomplete.unavailable == false {
            let res = self.database.delete_guild(incomplete.id).await;
            self.process_result(&ctx, res, None).await;
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        if is_new == Some(true) {
            let res = self.database.ensure_guild_exists(guild.id).await;
            self.process_result(&ctx, res, None).await;
        }
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

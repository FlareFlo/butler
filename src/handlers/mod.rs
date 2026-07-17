use std::sync::LazyLock;
use chrono::Utc;
use dashmap::DashMap;
use crate::commands::Data;
use crate::Config;
use poise::async_trait;
use serenity::all::{ActivityData, ChannelId, Context, EventHandler, Guild, GuildId, Message, MessageId, Ready, UnavailableGuild, UserId};
use tracing::info;

mod account_age;
mod honeypot;

pub struct Handler {
    pub database: Data,
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        handle_dm(ctx.clone(), &msg).await;
        let res = self.check_account_age_from_message(&ctx, &msg).await;
        self.process_result(&ctx, res, msg.guild_id).await;
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

pub static MSG_CACHE: LazyLock<DashMap<(GuildId, UserId, ChannelId), Vec<MessageId>>>  = LazyLock::new(||DashMap::<(GuildId, UserId, ChannelId), Vec<MessageId>>::new());

pub fn evict_stale_cache_entries() {
    let cutoff = Utc::now() - chrono::Duration::hours(1);
    MSG_CACHE.retain(|_key, messages| {
        messages.retain(|msg_id| *msg_id.created_at() > cutoff);
        !messages.is_empty()
    });
}

async fn handle_dm(ctx: Context, msg: &Message) {
    // Ignore non-DMs
    if let Some(gid) = msg.guild_id {
        MSG_CACHE
            .entry((gid, msg.author.id, msg.channel_id))
            .or_default()
            .push(msg.id);
        return;
    }
    if ctx.cache.current_user().id == msg.author.id {
        return;
    }
    info!("DM from {}", msg.author.name);
}

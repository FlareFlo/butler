mod account_age;
mod util;
mod honeypot;

use crate::account_age::check_account_age;
use serde::Deserialize;
use serenity::all::Message;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use std::fs;
use std::process::exit;
use std::sync::LazyLock;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use uptime_kuma_pusher::UptimePusher;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub min_hours: u64,
    pub uk_url: String,
    pub log_chat: u64,
    pub honeypot_channels: Vec<u64>,
    pub honeypot_safe_roles: Vec<u64>,
}

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        check_account_age(&ctx, &new_member).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        handle_dm(ctx.clone(), &msg).await;
        honeypot::handle_honeypot(ctx.clone(), &msg).await;
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

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    toml::from_str::<Config>(&fs::read_to_string("config.toml").unwrap()).unwrap()
});

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::builder().finish())
        .expect("tracing setup failed");

    ctrlc::set_handler(move || {
        error!("Got shutdown signal");
        exit(1);
    })
    .unwrap();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::MESSAGE_CONTENT;
    UptimePusher::new(&CONFIG.uk_url, true).spawn_background();

    let mut client = Client::builder(&CONFIG.token, intents)
        .event_handler(Handler {})
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

mod account_age;
mod error;
mod honeypot;
mod util;
mod db;

use color_eyre::Report;
use serde::Deserialize;
use serenity::all::Message;
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::process::exit;
use std::{env, fs};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use uptime_kuma_pusher::UptimePusher;

pub type ButlerResult<T> = Result<T, Report>;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub min_hours: u64,
    pub uk_url: String,
    pub log_chat: u64,
}

struct Handler {
    pub pool: PgPool,
    pub config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let res = self.check_account_age(&ctx, &new_member).await;
        self.process_result(&ctx, res).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        handle_dm(ctx.clone(), &msg).await;
        let res = self.handle_honeypot(ctx.clone(), &msg).await;
        self.process_result(&ctx, res).await;
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

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> ButlerResult<()> {
    color_eyre::install()?;
    tracing::subscriber::set_global_default(FmtSubscriber::builder().finish())?;

    ctrlc::set_handler(move || {
        error!("Got shutdown signal");
        exit(1);
    })?;

    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("missing DATABSE_URL env var"))
        .await?;

    MIGRATOR.run(&pool).await?;
    let config =
        toml::from_str::<Config>(&fs::read_to_string("config.toml")?)?;

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::MESSAGE_CONTENT;
    UptimePusher::new(&config.uk_url, true).spawn_background();

    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler { pool, config })
        .await
        .expect("Err creating client");

    client.start().await?;
    Ok(())
}

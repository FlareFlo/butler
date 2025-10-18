mod commands;
mod db;
mod error;
mod handlers;
mod serenity_ext;
mod util;

use crate::commands::Data;
use crate::commands::logging_channel::logging_channel;
use color_eyre::Report;
use handlers::Handler;
use serde::Deserialize;
use serenity::prelude::*;
use sqlx::PgPool;
use sqlx::migrate::Migrator;
use std::process::exit;
use std::{env, fs};
use tracing::error;
use tracing_subscriber::FmtSubscriber;
use uptime_kuma_pusher::UptimePusher;

pub type ButlerResult<T> = Result<T, Report>;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub min_hours: u64,
    pub uk_url: String,
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

    let pool =
        PgPool::connect(&env::var("DATABASE_URL").expect("missing DATABASE_URL env var")).await?;

    MIGRATOR.run(&pool).await?;
    let config = toml::from_str::<Config>(&fs::read_to_string("config.toml")?)?;

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::MESSAGE_CONTENT;
    if config.uk_url != "disabled" {
        UptimePusher::new(&config.uk_url, true).spawn_background();
    }

    let poise_pool = pool.clone();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![logging_channel()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool: poise_pool })
            })
        })
        .build();

    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler {
            database: commands::Data { pool },
            config,
        })
        .framework(framework)
        .await
        .expect("Err creating client");

    client.start().await?;
    Ok(())
}

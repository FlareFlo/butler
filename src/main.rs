use serde::Deserialize;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use std::fs;
use std::sync::LazyLock;
use std::time::Duration;
use serenity::all::{ChannelId, CreateMessage};
use uptime_kuma_pusher::UptimePusher;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub min_hours: i64,
    pub uk_url: String,
    pub log_chat: u64,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let user = &new_member.user;

        let created_at = user.created_at();
        println!("{} just joined, their account was created at: {}", user.name, created_at);

        let now = chrono::Utc::now();
        if (now - *created_at).num_hours() < CONFIG.min_hours {
            if let Err(err) = new_member
                .kick_with_reason(
                    &ctx.http,
                    &format!(
                        "Your account must be at least {} old",
                        humantime::format_duration(Duration::from_secs(
                            (CONFIG.min_hours * 60 * 60) as _
                        ))
                    ),
                )
                .await
            {
                println!("Failed to kick {}: {:?}", user.name, err);
            } else {
                println!("Kicked {} for being too new!", user.name);
                let m = CreateMessage::new().content(format!("Kicked {} for being too new!", user.name));
                if let Err(e) = ChannelId::new(CONFIG.log_chat).send_message(ctx.http, m).await {
                    dbg!(e);
                };
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    toml::from_str::<Config>(&fs::read_to_string("config.toml").unwrap()).unwrap()
});

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;
    UptimePusher::new(&CONFIG.uk_url, true).spawn_background();

    let mut client = Client::builder(&CONFIG.token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

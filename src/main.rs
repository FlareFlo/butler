use serde::Deserialize;
use serenity::all::{ChannelId, CreateMessage, Message};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use std::fs;
use std::ops::Add;
use std::sync::LazyLock;
use std::time::Duration;
use uptime_kuma_pusher::UptimePusher;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub min_hours: u64,
    pub uk_url: String,
    pub log_chat: u64,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let user = &new_member.user;

        let created_at = user.created_at();
        println!(
            "{} just joined, their account was created at: {}",
            user.name, created_at
        );

        // Skip if user is old enough
        let now = chrono::Utc::now();
        if (now - *created_at).num_hours() > CONFIG.min_hours as _ {
            return;
        }

        if let Err(err) = new_member
            .kick_with_reason(&ctx.http, "Kicked for brand new account")
            .await
        {
            println!("Failed to kick {}: {:?}", user.name, err);
        } else {
            println!("Kicked {} for being too new!", user.name);
        }

        // Log the kick
        let log_message =
            CreateMessage::new().content(format!("Kicked {} for being too new!", user.name));
        if let Err(e) = ChannelId::new(CONFIG.log_chat)
            .send_message(&ctx.http, log_message)
            .await
        {
            dbg!(e);
        };

        // DM user for kick reason
        let good_on = created_at
            .add(Duration::from_secs(CONFIG.min_hours * 60 * 60))
            .timestamp();
        let user_message = CreateMessage::new().content(format!(
            "Your account must be at least {} old.\nYou may rejoin on <t:{good_on}:f>\nDO NOT REPLY TO THIS MESSAGE, IT IS AUTOMATED AND WILL NOT BE READ OR RESPONDED TO!",
            humantime::format_duration(Duration::from_secs(
                CONFIG.min_hours * 60 * 60
            ))
        ));
        user.direct_message(&ctx.http, user_message).await.unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        // Ignore non-DMs
        if msg.guild_id.is_some() {
            return;
        }
        eprintln!("{} said {}", msg.author.name, msg.content);
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    toml::from_str::<Config>(&fs::read_to_string("config.toml").unwrap()).unwrap()
});

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    UptimePusher::new(&CONFIG.uk_url, true).spawn_background();

    let mut client = Client::builder(&CONFIG.token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

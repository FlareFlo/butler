use serde::Deserialize;
use serenity::all::{ChannelId, CreateMessage, Message};
use serenity::async_trait;
use serenity::gateway::ActivityData;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::prelude::*;
use std::fs;
use std::ops::Add;
use std::process::exit;
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

        // DM user for kick reason, happens before kick because it cannot talk to users
        let good_on = created_at
            .add(Duration::from_secs(CONFIG.min_hours * 60 * 60))
            .timestamp();
        let user_message = CreateMessage::new().content(format!(
            "Your account must be at least {} old.\nYou may rejoin on <t:{good_on}:f>\nDO NOT REPLY TO THIS MESSAGE, IT IS AUTOMATED AND WILL NOT BE READ OR RESPONDED TO!",
            humantime::format_duration(Duration::from_secs(
                CONFIG.min_hours * 60 * 60
            ))
        ));
        if let Err(e) = user.direct_message(&ctx.http, user_message).await {
            dbg!(e);
        }

        let reason = format!(
            "Kicked {} <@{}>\nAccount created on: {}\nVerification status: {}",
            user.name,
            user.id,
            created_at,
            user.verified
                .map(|e| e.to_string())
                .unwrap_or_else(|| "N/A".to_owned())
        );

        // Kick them
        if let Err(err) = new_member.kick_with_reason(&ctx.http, &reason).await {
            println!("Failed to kick {}: {:?}", user.name, err);
        } else {
            println!("Kicked {} for being too new!", user.name);
        }

        // Log the kick
        let log_message = CreateMessage::new().content(&reason);
        if let Err(e) = ChannelId::new(CONFIG.log_chat)
            .send_message(&ctx.http, log_message)
            .await
        {
            dbg!(e);
        };
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore non-DMs
        if msg.guild_id.is_some() {
            return;
        }
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }
        eprintln!("{} said {}", msg.author.name, msg.content);
    }

    async fn ready(&self, cx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        cx.set_activity(Some(ActivityData::watching("for bad actors")))
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    toml::from_str::<Config>(&fs::read_to_string("config.toml").unwrap()).unwrap()
});

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        exit(1);
    })
    .unwrap();

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

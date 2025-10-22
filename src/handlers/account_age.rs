use crate::ButlerResult;
use crate::db::action_journal::ModerationAction;
use crate::handlers::Handler;
use serenity::all::{Context, CreateMessage, Member};
use std::ops::Add;
use tracing::{info, warn};

impl Handler {
    pub async fn check_account_age(&self, ctx: &Context, new_member: &Member) -> ButlerResult<()> {
        let user = &new_member.user;

        let Some(min_age) = self
            .database
            .get_minimum_account_age(new_member.guild_id)
            .await?
        else {
            // Minimum hours are not configured/enabled
            return Ok(());
        };

        let created_at = user.created_at();
        info!(
            "{} just joined, their account was created at: {}",
            user.name, created_at
        );

        // Skip if user is old enough
        let now = chrono::Utc::now();
        if (now - *created_at).as_seconds_f64() > min_age.as_secs_f64() {
            return Ok(());
        }

        // DM user for kick reason, happens before kick because it cannot talk to users
        let good_on = created_at.add(min_age).timestamp();
        let user_message = CreateMessage::new().content(format!(
            "Your account must be at least {} old.\nYou may rejoin on <t:{good_on}:f>\nDO NOT REPLY TO THIS MESSAGE, IT IS AUTOMATED AND WILL NOT BE READ OR RESPONDED TO!",
            humantime::format_duration(min_age)
        ));
        user.direct_message(&ctx.http, user_message).await?;

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
        new_member.kick_with_reason(&ctx.http, &reason).await?;
        warn!("Kicked {} for being too new!", user.name);

        self.database
            .log_action_to_journal(
                new_member.guild_id,
                user.id,
                ModerationAction::KickedAccountAge,
            )
            .await?;

        // Log the kick
        self.log_discord(&ctx, &reason, new_member.guild_id).await?;
        Ok(())
    }
}

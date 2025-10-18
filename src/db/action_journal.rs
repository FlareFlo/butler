use serenity::all::{GuildId, UserId};
use sqlx::{query, FromRow};
use time::UtcDateTime;
use crate::ButlerResult;
use crate::commands::Data;


#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "moderation_action", rename_all = "snake_case")]
pub enum ModerationAction {
	KickedHoneypot,
	KickedAccountAge,
}

pub struct ActionJournal {
	guild_id: GuildId,
	offender_id: UserId,
	action: ModerationAction,
	time: UtcDateTime,
}

impl Data {
	pub async fn log_action_to_journal(&self, guild: GuildId, offender: UserId, action: ModerationAction) -> ButlerResult<()> {
		// Timestamp created by DB
		query!("
			INSERT INTO action_journal (guild, offender_id, action)
			VALUES ($1, $2, $3::moderation_action)
		",
			guild.get() as i64,
			offender.get() as i64,
			action as ModerationAction,

		).execute(&self.pool).await?;

		Ok(())
	}
}
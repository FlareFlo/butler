use color_eyre::Report;
use serenity::all::{ChannelId, Guild, GuildId};
use sqlx::query;
use crate::ButlerResult;

type Context<'a> = poise::Context<'a, Data, Report>;

pub mod logging_channel;

pub struct Data {
	pub(crate) pool: sqlx::PgPool,
}

#[macro_export]
macro_rules! ensure_admin {
    ($member:expr, $ctx:expr) => {
		use crate::serenity_ext::SerenityExt;
		if !$member.has_admin($ctx) {
			$ctx.reply("This command requires a role with administration permissions").await?;
			return Ok(());
		}
	};
}

impl Data {
	pub async fn set_logging_channel(&self, ctx: &Context<'_>, channel: ChannelId, guild: GuildId)-> ButlerResult<()> {
		query!("
			UPDATE guilds
			SET logging_channel = $1
			WHERE id = $2
 		", channel.get() as i64, guild.get() as i64).execute(&ctx.data().pool).await?;
		Ok(())
	}

	pub async fn reset_logging_channel(&self, ctx: &Context<'_>, guild: GuildId)-> ButlerResult<()> {
		query!("
			UPDATE guilds
			SET logging_channel = NULL
			WHERE id = $1
 		", guild.get() as i64).execute(&ctx.data().pool).await?;
		Ok(())
	}
}
use crate::commands::Data;
use color_eyre::Report;
use poise::serenity_prelude::Channel;
use sqlx::query;

type Context<'a> = poise::Context<'a, Data, Report>;

#[poise::command(slash_command)]
pub async fn logging_channel(
	ctx: Context<'_>,
	#[description = "Channel to log messages to"] channel: Option<Channel>,
) -> Result<(), Report> {
	let Some(guild) = ctx.guild_id() else {
		ctx.reply("This command can only be used in guilds or channels.").await?;
		return Ok(());
	};

	if let Some(channel) = channel {
		query!("
			UPDATE guilds
			SET logging_channel = $1
			WHERE id = $2
 		", channel.id().get() as i64, guild.get() as i64).execute(&ctx.data().pool).await?;
		ctx.reply(format!("Set logging channel to {channel}")).await?;
	}
	else {
		query!("
			UPDATE guilds
			SET logging_channel = NULL
			WHERE id = $1
 		", guild.get() as i64).execute(&ctx.data().pool).await?;
		ctx.reply("Removed logging channel").await?;
	}
	Ok(())
}
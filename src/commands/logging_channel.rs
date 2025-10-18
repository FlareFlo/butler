use crate::ensure_admin;
use color_eyre::eyre::ContextCompat;
use crate::commands::{Context, Data};
use color_eyre::Report;
use poise::serenity_prelude::Channel;
use sqlx::query;


#[poise::command(slash_command)]
pub async fn logging_channel(
	ctx: Context<'_>,
	#[description = "Channel to log messages to"] channel: Option<Channel>,
) -> Result<(), Report> {
	let Some(guild) = ctx.guild_id() else {
		ctx.reply("This command can only be used in guilds or channels.").await?;
		return Ok(());
	};
	let author = ctx.author_member().await.context("Expect user to have roles set in guild")?;
	ensure_admin!(author, &ctx);

	if let Some(channel) = channel {
		ctx.data().set_logging_channel(&ctx, channel.id(), guild).await?;
		ctx.reply(format!("Set logging channel to {channel}")).await?;
	}
	else {
		ctx.data().reset_logging_channel(&ctx, guild).await?;
		ctx.reply("Removed logging channel").await?;
	}
	Ok(())
}
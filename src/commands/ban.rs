use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use serenity::all::{Channel, Member, User, UserId};
use crate::commands::PoiseContext;
use crate::db::action_journal::ModerationAction;
use crate::serenity_ext::SerenityExt;

#[poise::command(slash_command, required_permissions = "BAN_MEMBERS", guild_only)]
pub async fn ban(
	ctx: PoiseContext<'_>,
	#[description = "Target user to ban"] who: Member,
	#[description = "Reason sent to banned user"] msg: Option<String>,
	#[description = "Days of past messages that will be deleted"] dmd: Option<u8>,
) -> Result<(), Report> {
	let guild = ctx
		.guild_id()
		.context("Command should be guild only but guild_id was unset")?;

	let author = ctx.author_member().await.context("Author should be known")?;
	if !author.has_permission(&ctx, |p|p.ban_members()) {
		ctx.reply("you do not have banning permissions").await?;
		return Ok(());
	}

	let dmd = dmd.unwrap_or(0);

	if dmd > 7 {
		ctx.reply("dmd cannot be greater than 7 days").await?;
		return Ok(());
	}

	if let Some(message) = msg {
		if message.len() > 512 {
			ctx.reply("message cannot be longer than 512 characters").await?;
			return Ok(());
		}

		who.ban_with_reason(ctx.http(), dmd, message).await?;
	} else {
		who.ban(ctx.http(), dmd).await?;
	}

	ctx.data().log_action_to_journal(guild, who.user.id, ModerationAction::CommandBanned, Some(ctx.author().id)).await?;

	ctx.reply("Banned {who}").await?;

	Ok(())
}
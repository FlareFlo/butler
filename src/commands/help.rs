use crate::ButlerResult;
use crate::commands::PoiseContext;

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
	ctx: PoiseContext<'_>,
	#[description = "Specific command to show help about"] command: Option<String>,
) -> ButlerResult<()> {
	let config = poise::builtins::HelpConfiguration {
		..Default::default()
	};
	poise::builtins::help(ctx, command.as_deref(), config).await?;
	Ok(())
}
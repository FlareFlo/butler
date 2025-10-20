use crate::commands::PoiseContext;
use color_eyre::Report;
use color_eyre::eyre::ContextCompat;
use poise::serenity_prelude::Channel;

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
)]
pub async fn logging_channel(
    ctx: PoiseContext<'_>,
    #[description = "Channel to log messages to"] channel: Option<Channel>,
) -> Result<(), Report> {
    let Some(guild) = ctx.guild_id() else {
        ctx.reply("This command can only be used in guilds or channels.")
            .await?;
        return Ok(());
    };
    let author = ctx
        .author_member()
        .await
        .context("Expect user to have roles set in guild")?;

    if let Some(channel) = channel {
        ctx.data()
            .set_logging_channel(&ctx, channel.id(), guild)
            .await?;
        ctx.reply(format!("Set logging channel to {channel}"))
            .await?;
    } else {
        ctx.data().reset_logging_channel(&ctx, guild).await?;
        ctx.reply("Removed logging channel").await?;
    }
    Ok(())
}

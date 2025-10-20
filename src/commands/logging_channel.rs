use color_eyre::eyre::ContextCompat;
use crate::commands::PoiseContext;
use color_eyre::Report;
use poise::serenity_prelude::Channel;

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    guild_only,
)]
pub async fn logging_channel(
    ctx: PoiseContext<'_>,
    #[description = "Channel to log messages to"] channel: Option<Channel>,
) -> Result<(), Report> {
    let guild = ctx.guild_id().context("Command should be guild only but guild_id was unset")?;


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

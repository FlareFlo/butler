use crate::commands::PoiseContext;
use crate::commands::util::{channels_to_string, roles_to_string};
use color_eyre::Report;
use color_eyre::eyre::ContextCompat;

#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn get_server_config(ctx: PoiseContext<'_>) -> Result<(), Report> {
    let guild = ctx
        .guild_id()
        .context("Command should be guild only but guild_id was unset")?;

    let honeypot = ctx.data().get_honeypot_from_guild_id(guild).await?;
    let logging_channel = ctx.data().get_logging_channel(&ctx, guild).await?;

    if honeypot.is_none() && logging_channel.is_none() {
        ctx.reply("Nothing is configured yet.").await?;
        return Ok(());
    }

    let mut stats = "".to_string();
    if let Some(honeypot) = honeypot {
        stats.push_str(&format!(
            "Honeypots: {}\nSafe roles: {}\nArmed: {}\n",
            channels_to_string(honeypot.channel_ids.iter()),
            roles_to_string(honeypot.safe_role_ids.iter()),
            honeypot.enabled
        ));
    }

    if let Some(channel) = logging_channel {
        stats.push_str(&format!(
            "\nLogging channel: {}",
            channel.to_channel(ctx.http()).await?
        ));
    }

    ctx.reply(stats).await?;

    Ok(())
}

use crate::commands::PoiseContext;
use crate::ensure_admin;
use color_eyre::Report;
use color_eyre::eyre::ContextCompat;
use itertools::Itertools;

#[poise::command(slash_command)]
pub async fn get_server_config(ctx: PoiseContext<'_>) -> Result<(), Report> {
    let Some(guild) = ctx.guild_id() else {
        ctx.reply("This command can only be used in guilds or channels.")
            .await?;
        return Ok(());
    };
    let author = ctx
        .author_member()
        .await
        .context("Expect user to have roles set in guild")?;
    ensure_admin!(author, &ctx);

    let honeypot = ctx.data().get_honeypot_from_guild_id(guild).await?;
    let logging_channel = ctx.data().get_logging_channel(&ctx, guild).await?;

    let mut stats = "".to_string();
    if let Some(honeypot) = honeypot {
        let mut channels = vec![];
        for channel_id in honeypot.channel_ids {
            channels.push(format!("<#{channel_id}>"));
        }
        let mut roles = vec![];
        for safe_role_id in honeypot.safe_role_ids {
            roles.push(format!("<@&{safe_role_id}>"));
        }

        stats.push_str(&format!(
            "Honeypots: {}\nSafe roles: {}\nArmed: {}\n",
            channels.into_iter().join(""),
            roles.into_iter().join(""),
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

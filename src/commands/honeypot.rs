use crate::commands::PoiseContext;
use crate::commands::util::roles_to_string;
use color_eyre::Report;
use color_eyre::eyre::ContextCompat;
use poise::serenity_prelude::Channel;
use serenity::all::Role;
use std::iter::once;

#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn setup_honeypot(
    ctx: PoiseContext<'_>,
    #[description = "Honeypot channel"] honeypot: Channel,
    #[description = "Safe role that will not be acted upon when typing in the honeypot"] safe_role: Role,
    #[description = "Enables the honeypot, defaults to armed state"] enabled: Option<bool>,
) -> Result<(), Report> {
    let guild = ctx
        .guild_id()
        .context("Command should be guild only but guild_id was unset")?;

    ctx.data()
        .set_honeypot_for_guild(
            guild,
            once(honeypot.id()),
            once(safe_role.id),
            enabled.unwrap_or(true),
        )
        .await?;

    ctx.reply("Successfully set honeypots").await?;

    Ok(())
}

#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn add_safe_role(
    ctx: PoiseContext<'_>,
    #[description = "Safe role that will not be acted upon when typing in the honeypot"] safe_role: Role,
) -> Result<(), Report> {
    let guild = ctx
        .guild_id()
        .context("Command should be guild only but guild_id was unset")?;

    let honeypot = ctx.data().add_safe_role(guild, safe_role.id).await?;

    if let Some(success) = honeypot {
        ctx.reply(format!(
            "Safe roles are now {}",
            roles_to_string(
                success
                    .safe_role_ids
                    .iter()
            )
        ))
        .await?;
    } else {
        ctx.reply("Failed to add safe role. Is the honeypot set up already?")
            .await?;
    }

    Ok(())
}

#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS", guild_only)]
pub async fn remove_safe_role(
    ctx: PoiseContext<'_>,
    #[description = "Safe role that will not be acted upon when typing in the honeypot"] safe_role: Role,
) -> Result<(), Report> {
    let guild = ctx
        .guild_id()
        .context("Command should be guild only but guild_id was unset")?;

    let honeypot = ctx.data().remove_safe_role(guild, safe_role.id).await?;

    if let Some(success) = honeypot {
        ctx.reply(format!(
            "Safe roles are now {}",
            roles_to_string(
                success
                    .safe_role_ids
                    .iter()
            )
        ))
        .await?;
    } else {
        ctx.reply("Failed to add safe role. Is the honeypot set up already?")
            .await?;
    }

    Ok(())
}

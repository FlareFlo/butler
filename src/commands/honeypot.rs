use std::iter::once;
use crate::commands::PoiseContext;
use color_eyre::Report;
use poise::serenity_prelude::Channel;
use serenity::all::Role;

#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
)]
pub async fn setup_honeypot(
    ctx: PoiseContext<'_>,
    #[description = "Honeypot channel"] honeypot: Channel,
    #[description = "Safe role"] safe_role: Role,
    #[description = "Enabled"] enabled: bool,
) -> Result<(), Report> {
    let Some(guild) = ctx.guild_id() else {
        ctx.reply("This command can only be used in guilds or channels.")
            .await?;
        return Ok(());
    };

    ctx.data()
        .set_honeypot_for_guild(
            guild,
            once(honeypot.id()),
            once(safe_role.id),
            enabled,
        )
        .await?;

    ctx.reply("Successfully set honeypots").await?;

    Ok(())
}

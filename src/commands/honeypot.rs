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
    #[description = "Honeypot channels"] honeypots: Vec<Channel>,
    #[description = "Safe roles"] safe_roles: Vec<Role>,
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
            honeypots.iter().map(|e| e.id()),
            safe_roles.iter().map(|e| e.id),
            enabled,
        )
        .await?;

    ctx.reply("Successfully set honeypots").await?;

    Ok(())
}

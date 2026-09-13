use crate::commands::PoiseContext;
use color_eyre::Report;

#[poise::command(slash_command, description_localized("en-US", "Show bot statistics"))]
pub async fn stats(ctx: PoiseContext<'_>) -> Result<(), Report> {
    let mut guild_count = 0;
    let mut user_count = 0;

    for guild_id in ctx.cache().guilds() {
        guild_count += 1;
        if let Some(guild) = ctx.cache().guild(guild_id) {
            user_count += guild.member_count;
        }
    }

    let stats = format!("Protecting **{}** users across **{}** servers.", user_count, guild_count);
    ctx.reply(stats).await?;

    Ok(())
}

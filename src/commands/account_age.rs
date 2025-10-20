use crate::commands::PoiseContext;
use color_eyre::Report;
use color_eyre::eyre::ContextCompat;
use poise::serenity_prelude::Channel;
use std::time::Duration;

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR", guild_only)]
pub async fn set_minimum_account_age(
    ctx: PoiseContext<'_>,
    #[description = "Minimum account age in days"] minimum_account_age_days: Option<u64>,
) -> Result<(), Report> {
    let guild_id = ctx
        .guild_id()
        .context("Expected to be invoked in a guild")?;
    if let Some(min) = minimum_account_age_days {
        if min > 28 {
            ctx.reply("The maximum possible account age is restricted to 28 days or less.\n\
            See why: <https://docs.rs/serenity/latest/serenity/model/guild/struct.Member.html#method.disable_communication_until_datetime>").await?;
            return Ok(());
        }

        let min = Duration::from_secs_f64(min as f64 * 24.0 * 60.0 * 60.0);
        ctx.data()
            .set_minimum_account_age(guild_id, Some(min))
            .await?;
        ctx.reply(format!(
            "Minimum account age has been set to {}",
            humantime::format_duration(min)
        ))
        .await?;
    } else {
        ctx.data().set_minimum_account_age(guild_id, None).await?;
        ctx.reply("Minimum account age has been disabled and unset")
            .await?;
    }
    Ok(())
}

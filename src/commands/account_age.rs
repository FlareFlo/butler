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
        ctx.data()
            .set_minimum_account_age(
                guild_id,
                Some(Duration::from_secs_f64(min as f64 / 24.0 / 60.0 / 60.0)),
            )
            .await?;
    } else {
        ctx.data().set_minimum_account_age(guild_id, None).await?;
    }
    Ok(())
}

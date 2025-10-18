use color_eyre::Report;
pub type PoiseContext<'a> = poise::Context<'a, Data, Report>;

pub mod config;
pub mod honeypot;
pub mod logging_channel;

pub struct Data {
    pub(crate) pool: sqlx::PgPool,
}

#[macro_export]
macro_rules! ensure_admin {
    ($member:expr, $ctx:expr) => {
        use crate::serenity_ext::SerenityExt;
        if !$member.has_admin($ctx) {
            $ctx.reply("This command requires a role with administration permissions")
                .await?;
            return Ok(());
        }
    };
}

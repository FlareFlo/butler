use color_eyre::Report;
pub type PoiseContext<'a> = poise::Context<'a, Data, Report>;

pub mod config;
pub mod honeypot;
pub mod logging_channel;

pub struct Data {
    pub(crate) pool: sqlx::PgPool,
}
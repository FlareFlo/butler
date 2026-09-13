use color_eyre::Report;
pub type PoiseContext<'a> = poise::Context<'a, Data, Report>;

pub mod account_age;
pub mod ban;
pub mod config;
pub mod help;
pub mod honeypot;
pub mod logging_channel;
pub mod stats;

pub use account_age::set_minimum_account_age;
pub use ban::ban;
pub use config::get_server_config;
pub use help::help;
pub use honeypot::{add_honeypot_channel, add_safe_role, remove_honeypot_channel, remove_safe_role, setup_honeypot};
pub use logging_channel::logging_channel;
pub use stats::stats;
mod util;

pub struct Data {
    pub(crate) pool: sqlx::PgPool,
}

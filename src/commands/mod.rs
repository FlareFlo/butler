pub mod logging_channel;

pub struct Data {
	pub(crate) pool: sqlx::PgPool,
}
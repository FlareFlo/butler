

#[derive(sqlx::FromRow)]
pub struct Honeypot {
	pub id: i64,
	pub channel_ids: Vec<i64>,
	pub safe_role_ids: Vec<i64>,
	pub enabled: bool,
}
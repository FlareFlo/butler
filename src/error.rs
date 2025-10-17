use crate::{ButlerResult, Handler};
use serenity::all::{Context, GuildId};
use tracing::error;

impl Handler {
    pub async fn process_result<T>(&self, ctx: &Context, res: ButlerResult<T>, guild_id: Option<GuildId>) {
        let Err(error) = res else {
            return;
        };
        let errstr = error.to_string();
        error!("{}", errstr);
        if let Some(guild_id) = guild_id {
            let err = self.log_discord(ctx, &errstr, guild_id).await;
            if let Err(err) = err {
                error!("{}", err.to_string());
            }
        }
    }
}

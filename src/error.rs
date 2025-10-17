use crate::{ButlerResult, Handler};
use serenity::all::Context;
use tracing::error;

impl Handler {
    pub async fn process_result<T>(&self, ctx: &Context, res: ButlerResult<T>) {
        let Err(error) = res else {
            return;
        };
        let errstr = error.to_string();
        error!("{}", errstr);
        self.log_discord(ctx, &errstr).await;
    }
}

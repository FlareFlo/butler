use crate::ButlerResult;
use crate::util::log_discord;
use serenity::all::Context;
use tracing::error;

pub async fn process_result<T>(ctx: &Context, res: ButlerResult<T>) {
    let Err(error) = res else {
        return;
    };
    let errstr = error.to_string();
    error!("{}", errstr);
    log_discord(ctx, &errstr).await;
}

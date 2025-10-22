#[macro_export]
macro_rules! process_result {
    ($self:expr, $ctx:expr, $res:expr, $guild_id:expr) => {
        let Err(error) = $res else {
            return;
        };
        let errstr = error.to_string();
        tracing::error!("{}", errstr);
        if let Some(guild_id) = $guild_id {
            let err = $self.log_discord($ctx, &errstr, guild_id).await;
            if let Err(err) = err {
                tracing::error!("Failed to log to discord: {}", err.to_string());
            }
        }
    };
}

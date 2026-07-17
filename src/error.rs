use std::fmt::{Debug, Display};
use tracing::error;

pub trait ButlerErrorExt {
    fn log_err(&self);
}

impl<T, E: Display> ButlerErrorExt for Result<T, E> {
    fn log_err(&self) {
        if let Err(err) = self {
            error!("{err}");
        }
    }
}

#![feature(error_generic_member_access, let_chains, never_type, concat_idents)]

use std::{backtrace::Backtrace, panic::Location};

use async_std::task;
use thiserror::Error;

mod css;
mod ui;

#[derive(Error, Debug)]
pub enum Error {
    #[error("At {location}: CSS Watcher Error:\n{source}")]
    CssWatcher {
        #[from]
        source: css::error::WatcherError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

#[async_std::main]
async fn main() -> Result<!, Error> {
    task::spawn_blocking(ui::run_blocking);
    css::watch(ui::get_app().await).await?;
}

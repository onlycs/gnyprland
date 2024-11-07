use std::backtrace::Backtrace;
use std::panic::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReloadError {
    #[error("At {location}: GLib error:\n{source}")]
    IoError {
        #[from]
        source: glib::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("At {location}: Watcher error:\n{source}")]
    WatcherError {
        #[from]
        source: notify::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Error reloading css:\n{source}")]
    ReloadError {
        #[from]
        source: ReloadError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

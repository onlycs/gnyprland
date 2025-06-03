use std::{backtrace::Backtrace, env, io, panic::Location, string::FromUtf8Error};

use smol::channel::RecvError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventParseError {
    #[error("Event had invalid data or format")]
    InvalidData,

    #[error("Missing field: {0}")]
    MissingField(&'static str),

    #[error("Failed to parse field: {0}")]
    ParseFailed(&'static str),
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Failed to send command to thread")]
    SendFailed,

    #[error("At {location}: Receive error: {source}")]
    Recv {
        #[from]
        source: RecvError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Deserialize error: {source}")]
    Deserialize {
        #[from]
        source: serde_json::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Failed to get environment variable: {source}")]
    Env {
        #[from]
        source: env::VarError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: I/O Error: {source}")]
    Io {
        #[from]
        source: io::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub enum ListenError {
    #[error("At {location}: Failed to read env: {source}")]
    Env {
        #[from]
        source: env::VarError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: I/O Error: {source}")]
    IO {
        #[from]
        source: io::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Failed to convert bytes to string: {source}")]
    Utf8 {
        #[from]
        source: FromUtf8Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Failed to parse event: {source}")]
    Parse {
        #[from]
        source: EventParseError,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

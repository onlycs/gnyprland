use std::{
    backtrace::Backtrace,
    fs::OpenOptions,
    io::{self},
    panic::Location,
    path::Path,
    time::Instant,
};

use async_std::{
    channel, process,
    stream::StreamExt,
    task::{self, JoinHandle},
};
use notify::{
    recommended_watcher, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher,
};
use thiserror::Error;

pub const FILE: &str = "/tmp/gnyprland/index.css";

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("At {location}: I/O Error: {source}")]
    IO {
        #[from]
        source: io::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("At {location}: Notify Error: {source}")]
    Notify {
        #[from]
        source: notify::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

#[cfg(debug_assertions)]
pub fn begin_watch(reload: impl Fn() + Send + Sync + 'static) {
    task::block_on(
        process::Command::new("sass")
            .args(["styles/index.scss", FILE])
            .output(),
    )
    .unwrap();

    reload();

    task::spawn(async move {
        let (tx, mut rx) = channel::unbounded();
        let mut watcher = RecommendedWatcher::new(
            move |res| task::block_on(tx.send(res)).unwrap(),
            Default::default(),
        )
        .unwrap();

        watcher
            .watch(Path::new("styles"), notify::RecursiveMode::Recursive)
            .unwrap();

        loop {
            let event = rx.next().await;

            let Some(Ok(event)) = event else {
                continue;
            };

            if !matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
            ) {
                continue;
            }

            process::Command::new("sass")
                .args(["styles/index.scss", FILE])
                .output()
                .await
                .unwrap();

            reload();
        }
    });
}

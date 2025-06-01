use std::{
    backtrace::Backtrace,
    io::{self},
    panic::Location,
    path::Path,
};

use notify::{EventKind, RecommendedWatcher, Watcher};
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
    use smol::{channel, process};

    smol::block_on(
        process::Command::new("sass")
            .args(["styles/index.scss", FILE])
            .output(),
    )
    .unwrap();

    reload();

    smol::spawn(async move {
        let (tx, rx) = channel::unbounded();
        let mut watcher = RecommendedWatcher::new(
            move |res| smol::block_on(tx.send(res)).unwrap(),
            Default::default(),
        )
        .unwrap();

        watcher
            .watch(Path::new("styles"), notify::RecursiveMode::Recursive)
            .unwrap();

        loop {
            let event = rx.recv().await;

            let Ok(Ok(event)) = event else {
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
    })
    .detach();
}

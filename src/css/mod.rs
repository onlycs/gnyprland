pub mod error;

use crate::prelude::*;
use async_std::{process::Command, stream::StreamExt, task};
use error::{ReloadError, WatcherError};
use futures::{channel::mpsc, SinkExt};
use notify::{EventKind, RecommendedWatcher, Watcher};
use std::{
    path::Path,
    time::{Duration, Instant},
};

type Result<T, E = WatcherError> = std::result::Result<T, E>;

async fn recompile() -> Result<()> {
    let output = Command::new("sass")
        .args(&["styles/index.scss", "/tmp/ags/index.css"])
        .output()
        .await
        .unwrap();

    match output.status.code() {
        Some(0) => println!("Successfully compiled styles"),
        Some(code) => {
            eprintln!("Failed to compile styles: exit code {}", code);
            eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        }
        None => eprintln!("Failed to compile styles: no exit code"),
    }

    Ok(())
}

pub fn reload(app: &Application) -> Result<(), ReloadError> {
    app.apply_css("/tmp/ags/index.css", true);
    Ok(())
}

pub async fn watch(app: &'static Application) -> Result<!> {
    task::block_on::<_, Result<()>>(recompile())?;
    reload(app)?;

    let (mut tx, mut rx) = mpsc::unbounded();
    let mut watcher = RecommendedWatcher::new(
        move |res| task::block_on(tx.send(res)).unwrap(),
        Default::default(),
    )?;

    let mut timeout = Instant::now();
    watcher.watch(Path::new("styles"), notify::RecursiveMode::Recursive)?;

    while let Some(event) = rx.next().await {
        let event = event?;

        if matches!(event.kind, EventKind::Other) {
            break;
        }

        if !matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            continue;
        }

        if timeout.elapsed() <= Duration::from_millis(500) {
            continue;
        } else {
            timeout = Instant::now();
        }

        task::block_on::<_, Result<()>>(recompile())?;
        reload(app)?;
    }

    panic!("watcher exited unexpectedly")
}

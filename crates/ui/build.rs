#![feature(exit_status_error)]

use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let is_release = env::var("PROFILE").unwrap() == "release";

    if is_release {
        let _ = process::Command::new("sass")
            .args([
                "../../styles/index.scss",
                "/tmp/gnyprland/index-release.css",
            ])
            .output()?
            .exit_ok()?;
    }

    Ok(())
}

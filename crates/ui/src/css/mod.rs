use crate::prelude::*;

cfg_if! {
    if #[cfg(debug_assertions)] {
        mod watch;
        pub use watch::*;
    } else {
        mod env;

        use std::{io::{self, Write}, fs::OpenOptions};
        pub use env::*;
    }
}

#[cfg(not(debug_assertions))]
pub fn write_css() -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE)?;

    file.write_all(CSS.as_bytes())?;

    Ok(())
}

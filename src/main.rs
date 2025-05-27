#![feature(error_generic_member_access)]

extern crate async_std;
#[macro_use]
extern crate cfg_if;
extern crate hyprland;
extern crate lazy_static;
extern crate relm4;
extern crate thiserror;
#[macro_use]
extern crate log;

mod bar;
mod css;
mod prelude;

use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_threads(true)
        .with_local_timestamps()
        .with_timestamp_format(time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .with_level(if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init()
        .unwrap();

    bar::run();
}

#![feature(error_generic_member_access)]

#[macro_use]
extern crate cfg_if;
extern crate gnyprland_relay;
extern crate gtk4_layer_shell;
extern crate hyprland;
#[macro_use]
extern crate log;
extern crate map_macro;
extern crate notify;
extern crate relm4;
extern crate smol;
extern crate thiserror;

mod bar;
mod css;
mod prelude;

use bar::Bar;
use gnyprland_relay::message::IpcReceiver;
use relm4::RelmApp;

pub fn start(receiver: IpcReceiver) {
    let app = RelmApp::new("page.angad.gnyprland");
    app.allow_multiple_instances(false);
    app.run::<Bar>(receiver);
}

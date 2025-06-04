#![feature(error_generic_member_access, decl_macro, iter_array_chunks)]

extern crate cfg_if;
extern crate gnyprland_relay;
extern crate gtk4_layer_shell;
extern crate hyprland;
extern crate log;
extern crate map_macro;
extern crate notify;
extern crate relm4;
extern crate smol;
extern crate thiserror;
extern crate zbus;

mod bar;
mod center_menu;
mod css;
mod overlays;
mod prelude;

use bar::Bar;
use gnyprland_relay::message::IpcReceiver;
use relm4::RelmApp;

pub fn start(receiver: IpcReceiver) {
    let bar = RelmApp::new("gnyprland.bar");
    bar.allow_multiple_instances(false);
    bar.run::<Bar>(receiver);
}

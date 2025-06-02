pub use std::{
    convert::identity,
    thread,
    time::{Duration, Instant},
};

pub use cfg_if::cfg_if;
pub use gtk::{self, glib::clone, prelude::*};
pub use gtk4_layer_shell::{Edge, Layer, LayerShell};
pub use hyprland::event_listener::EventListener;
pub use log::*;
pub use map_macro::hash_map;
pub use relm4::prelude::*;

pub macro css {
    (inner $vec:ident, $string:literal) => {
        $vec.extend($string.split_whitespace());
    },

    (inner $vec:ident, $string:literal if $cond:expr) => {
        if $cond {
            $vec.extend($string.split_whitespace());
        }
    },

    (inner $vec:ident, $string:literal if $cond:expr, $($rest:tt)*) => {
        if $cond {
            $vec.extend($string.split_whitespace());
            css!(inner $vec, $($rest)*)
        } else {
            css!(inner $vec, $($rest)*)
        }
    },

    (inner $vec:ident, $string:literal, $($rest:tt)*) => {
        $vec.extend($string.split_whitespace());
        css!(inner $vec, $($rest)*)
    },

    ($($rest:tt)*) => {
        {
            let mut css = Vec::new();
            css!(inner css, $($rest)*);
            css
        }
        .as_slice()
    }
}

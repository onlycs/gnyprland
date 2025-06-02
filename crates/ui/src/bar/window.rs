use std::{collections::HashMap, sync::LazyLock};

use hyprland::{data::Client, event_listener::WindowEventData, prelude::*, shared::Address};
use relm4::gtk::{pango::EllipsizeMode, Orientation};

use crate::prelude::*;

type ClassMap = HashMap<&'static str, fn(String) -> String>;

static TITLE_OVERRIDES: LazyLock<ClassMap> = LazyLock::new(|| {
    hash_map! {
        "kitty" => (|_| String::from("Terminal")) as fn(String) -> String,
        "code" => |title| title.replace(" - Visual Studio Code", ""),
        "firefox" => |title| title.replace("Mozilla Firefox", "Firefox"),
        "" => |_| String::from("Desktop")
    }
});

static CLASS_OVERRIDES: LazyLock<ClassMap> = LazyLock::new(|| {
    hash_map! {
        "dev.zed.Zed" => (|_| String::from("zed")) as fn(String) -> String,
    }
});

fn title_override(class: &str, title: String) -> String {
    TITLE_OVERRIDES.get(class).copied().unwrap_or(identity)(title)
}

fn class_override(class: String) -> String {
    CLASS_OVERRIDES
        .get(class.as_str())
        .copied()
        .unwrap_or(identity)(class)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message {
    Update(WindowEventData),
}

pub struct ActiveWindow {
    active: WindowEventData,
}

#[relm4::component(pub)]
impl SimpleComponent for ActiveWindow {
    type Init = ();
    type Input = Message;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: Orientation::Vertical,
            set_css_classes: css!["element", "active-window"],

            gtk::Label {
                #[watch]
                set_label: &title_override(&model.active.class, model.active.title.clone()),
                set_css_classes: &["text"],
                set_max_width_chars: 10,
                set_ellipsize: EllipsizeMode::End
            },

            gtk::Label {
                #[watch]
                set_label: &class_override(model.active.class.clone()),
                set_css_classes: &["text-sub"],
                set_max_width_chars: 10,
                set_ellipsize: EllipsizeMode::End,
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        thread::spawn(move || {
            let mut listener = EventListener::new();

            listener.add_active_window_changed_handler(move |event| {
                let Some(window) = event else {
                    info!("No active window found");
                    return;
                };

                trace!(
                    "Active window changed: {} (class: {})",
                    window.title,
                    window.class
                );

                sender.input(Message::Update(window));
            });

            debug!("Watching for active window changes");
            listener.start_listener().unwrap()
        });

        let current_active = match Client::get_active().unwrap() {
            Some(client) => WindowEventData {
                address: client.address,
                title: client.title,
                class: client.class,
            },
            None => WindowEventData {
                address: Address::new("unknown"),
                class: String::new(),
                title: String::new(),
            },
        };

        let model = ActiveWindow {
            active: current_active,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        match message {
            Message::Update(window) => self.active = window,
        }
    }
}

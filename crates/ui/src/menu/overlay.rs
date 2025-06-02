use gtk4_layer_shell::KeyboardMode;
use relm4::gtk::{gdk::Key, glib::Propagation, EventControllerKey, GestureClick};

use crate::{menu::open::OpenMenu, prelude::*};

pub struct Overlay {
    visible: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for Overlay {
    type Init = ();
    type Input = bool;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("overlay"),
            set_css_classes: css!["overlay"],

            init_layer_shell: (),
            set_layer: Layer::Top,
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Bottom, true),
            set_anchor: (Edge::Left, true),
            set_keyboard_mode: KeyboardMode::OnDemand,
            set_exclusive_zone: -1,

            #[watch]
            set_visible: model.visible,

            add_controller: gesture,
            add_controller: key,
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        OpenMenu::on_change(sender.input_sender(), |open| open.is_some());

        let gesture = GestureClick::new();
        let key = EventControllerKey::new();

        gesture.connect_pressed(|_, _, _, _| {
            OpenMenu::set(None);
        });

        key.connect_key_pressed(|_, key, _, _| {
            if key == Key::Escape {
                OpenMenu::set(None);
            }

            Propagation::Stop
        });

        let model = Overlay { visible: false };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        self.visible = message;
    }
}

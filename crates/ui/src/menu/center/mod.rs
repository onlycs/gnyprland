use gtk4_layer_shell::KeyboardMode;

use super::open::OpenMenu;
use crate::prelude::*;

pub struct CenterMenu {
    open: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for CenterMenu {
    type Init = ();
    type Input = bool;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("center window"),
            set_css_classes: css!["center-window"],

            init_layer_shell: (),
            set_layer: Layer::Overlay,
            set_anchor: (Edge::Top, true),
            set_keyboard_mode: KeyboardMode::OnDemand,
            set_exclusive_zone: 0,

            #[watch]
            set_visible: model.open,
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        OpenMenu::on_change(sender.input_sender(), |open| {
            open == Some(OpenMenu::Calendar)
        });

        let model = CenterMenu { open: false };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, open: bool, _: ComponentSender<Self>) {
        self.open = open;
    }
}

mod notifications;

use gtk4_layer_shell::KeyboardMode;
use notifications::Notifications;

use crate::{overlays::active::ActiveOverlay, prelude::*};

pub struct CenterMenu {
    open: bool,

    notifications: Controller<Notifications>,
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

            #[local_ref]
            notifications_widget -> root!(Notifications),
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        ActiveOverlay::on_change(sender.input_sender(), |open| {
            open == Some(ActiveOverlay::Center)
        });

        let notifications = Notifications::builder();
        let notifications_widget = notifications.root.clone();

        let model = CenterMenu {
            open: false,
            notifications: notifications.launch(()).detach(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, open: bool, _: ComponentSender<Self>) {
        self.open = open;
    }
}

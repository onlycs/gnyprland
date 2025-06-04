use super::{model::Notification, UiMessage};
use crate::prelude::*;

pub struct NotificationWidget {
    notification: Notification,
}

#[relm4::component(pub)]
impl SimpleComponent for NotificationWidget {
    type Init = Notification;
    type Input = ();
    type Output = UiMessage;

    view! {
        gtk::Box {}
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();
        let model = Self { notification: init };

        ComponentParts { widgets, model }
    }
}

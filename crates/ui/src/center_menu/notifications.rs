mod daemon;
mod model;
mod widget;
mod worker;

use model::Notification;
use relm4::WorkerController;
use worker::NotificationDbusWorker;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiMessage {
    New(Notification),
    Close(u32),
    Timeout(u32),
    Action(u32, String),
}

pub struct Notifications {
    worker: WorkerController<NotificationDbusWorker>,
}

#[relm4::component(pub)]
impl SimpleComponent for Notifications {
    type Init = ();
    type Input = UiMessage;
    type Output = ();

    view! {
        gtk::Box {}
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let worker = NotificationDbusWorker::builder()
            .detach_worker(())
            .forward(sender.input_sender(), identity);

        let model = Self { worker };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("{message:?}");
    }
}

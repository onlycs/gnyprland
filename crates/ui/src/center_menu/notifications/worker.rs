use relm4::Worker;
use zbus::{connection, Connection};

use super::{daemon::NotificationDaemon, model::CloseReason, UiMessage};
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DbusMessage {
    Closed(u32, CloseReason),
    Action(u32, String),
}

pub struct NotificationDbusWorker {
    connection: Connection,
}

impl Worker for NotificationDbusWorker {
    type Init = ();
    type Input = DbusMessage;
    type Output = UiMessage;

    fn init(_: Self::Init, sender: relm4::ComponentSender<Self>) -> Self {
        let daemon = NotificationDaemon::new(sender.output_sender().clone());
        let connection = smol::block_on::<Result<_, zbus::Error>>(async move {
            connection::Builder::session()?
                .name("org.freedesktop.Notifications")?
                .serve_at("/org/freedesktop/Notifications", daemon)?
                .build()
                .await
        })
        .unwrap();

        Self { connection }
    }

    fn update(&mut self, message: Self::Input, _: relm4::ComponentSender<Self>) {
        match message {
            DbusMessage::Action(id, key) => smol::block_on(self.connection.call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "ActionInvoked",
                &(id, key),
            ))
            .unwrap(),
            DbusMessage::Closed(id, reason) => smol::block_on(self.connection.call_method(
                Some("org.freedesktop.Notifications"),
                "/org/freedesktop/Notifications",
                Some("org.freedesktop.Notifications"),
                "NotificationClosed",
                &(id, reason as u32),
            ))
            .unwrap(),
        };
    }
}

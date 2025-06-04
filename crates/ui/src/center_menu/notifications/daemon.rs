use std::{
    collections::HashMap,
    sync::{
        atomic::{self, AtomicU32},
        PoisonError, RwLock,
    },
    thread,
    time::Duration,
};

use relm4::Sender;
use zbus::{interface, zvariant};

use super::{
    model::{Action, Notification},
    UiMessage,
};

pub struct NotificationDaemon {
    ids: RwLock<Vec<u32>>,
    counter: AtomicU32,
    tx: Sender<UiMessage>,
}

impl NotificationDaemon {
    pub fn new(tx: Sender<UiMessage>) -> Self {
        Self {
            ids: RwLock::new(Vec::new()),
            counter: AtomicU32::new(1),
            tx,
        }
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationDaemon {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<String>,
        _hints: HashMap<String, zvariant::Value>,
        expire_timeout: i32,
    ) -> u32 {
        let id;

        if replaces_id == 0 {
            if let Some(next) = self
                .ids
                .write()
                .unwrap_or_else(PoisonError::into_inner)
                .pop()
            {
                id = next;
            } else {
                id = self.counter.fetch_add(1, atomic::Ordering::Relaxed);
            }
        } else {
            id = replaces_id;
        }

        let notification = Notification {
            id,
            app_name: app_name.to_string(),
            app_icon: app_icon.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            actions: Action::parse(actions),
            expiry: u32::try_from(expire_timeout).ok(),
        };

        self.tx.emit(UiMessage::New(notification));

        if expire_timeout > 0 {
            let tx = self.tx.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(expire_timeout as u64));
                tx.emit(UiMessage::Timeout(id));
            });
        }

        id
    }

    fn get_capabilities(&self) -> Vec<&str> {
        vec!["body", "actions"]
    }

    fn get_server_information(&self) -> (String, String, String, String) {
        (
            "Gnyprland".into(),
            "Angad".into(),
            "1.0".into(),
            "1.2".into(),
        )
    }
}

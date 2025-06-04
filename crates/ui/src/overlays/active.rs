use std::sync::{Arc, LazyLock, RwLock};

use relm4::Sender;

type Callback = Arc<dyn Fn(Option<ActiveOverlay>) + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActiveOverlay {
    Center,
}

impl ActiveOverlay {
    fn callbacks<'a>() -> &'a RwLock<Vec<Callback>> {
        static CALLBACKS: LazyLock<Arc<RwLock<Vec<Callback>>>> =
            LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

        CALLBACKS.as_ref()
    }

    pub fn on_change<M: Send + Sync + 'static>(
        sender: &Sender<M>,
        f: fn(Option<ActiveOverlay>) -> M,
    ) {
        let mut callbacks = Self::callbacks().write().unwrap();
        let sender = sender.clone();

        callbacks.push(Arc::new(move |value| {
            sender.emit(f(value));
        }));
    }

    pub fn set(value: Option<ActiveOverlay>) {
        let callbacks = Self::callbacks().read().unwrap();

        for callback in callbacks.iter() {
            callback(value);
        }
    }
}

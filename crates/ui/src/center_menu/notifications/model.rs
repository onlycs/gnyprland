use serde_repr::Serialize_repr;
use zbus::zvariant::Type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Type)]
#[repr(u32)]
pub enum CloseReason {
    Expired = 1,
    Dismissed = 2,
    Forced = 3,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub key: String,
    pub title: String,
}

impl Action {
    fn new([key, title]: [String; 2]) -> Self {
        Self { key, title }
    }

    pub fn parse(actions: Vec<String>) -> Vec<Action> {
        actions
            .into_iter()
            .array_chunks::<2>()
            .map(Action::new)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<Action>,
    pub expiry: Option<u32>,
}

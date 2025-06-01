use strum::{Display, EnumString};

use super::{RelayReceiver, RelaySender};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
pub enum IpcMessage {
    #[strum(serialize = "inspector")]
    StartInspector,
    #[strum(serialize = "reload-css")]
    ReloadCSS,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum IpcResponse {
    #[strum(to_string = "ok")]
    Ok,
    #[strum(to_string = "Inspector is only available in debug mode")]
    InspectorNotAvailable,
}

pub type IpcSender = RelaySender<IpcMessage, IpcResponse>;
pub type IpcReceiver = RelayReceiver<IpcMessage, IpcResponse>;

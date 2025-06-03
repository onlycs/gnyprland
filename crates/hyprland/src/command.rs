use std::env;

use serde::Deserialize;
use smol::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::unix::UnixStream,
};

use crate::error::CommandError;

pub trait Command: Sized + Send + Sync + 'static {
    type Response: for<'de> Deserialize<'de> + Send + Sync;

    const NAME: &'static str;
}

pub enum Executor {}

impl Executor {
    fn socket() -> Result<String, env::VarError> {
        let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
        let xrd = env::var("XDG_RUNTIME_DIR")?;
        Ok(format!("{xrd}/hypr/{his}/.socket.sock"))
    }

    pub async fn command_async<C: Command>() -> Result<C::Response, CommandError> {
        let socket = Self::socket()?;
        let mut stream = UnixStream::connect(socket).await?;

        stream
            .write_all(format!("-j/{}", C::NAME).as_bytes())
            .await?;

        let mut buf = String::new();
        stream.read_to_string(&mut buf).await?;

        let res = serde_json::from_str(&buf)?;

        Ok(res)
    }

    pub fn command<C: Command>() -> Result<C::Response, CommandError> {
        smol::block_on(Self::command_async::<C>())
    }
}

macro_rules! command {
    ($name:ident($strname:literal) => $return:ty) => {
        pub enum $name {}

        impl Command for $name {
            type Response = $return;

            const NAME: &str = $strname;
        }
    };

    ($($name:ident($strname:literal) => $return:ty),* $(,)?) => {
        $(command!($name($strname) => $return);)*
    };
}

#[derive(Clone, Debug, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub windows: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Client {
    pub class: String,
    pub title: String,
}

command!(
    Workspaces("workspaces") => Vec<Workspace>,
    Clients("clients") => Vec<Client>,
    ActiveWindow("activewindow") => Client,
    ActiveWorkspace("activeworkspace") => Workspace,
);

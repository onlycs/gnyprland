#![feature(cfg_select)]

extern crate clap;
extern crate simple_logger;
#[macro_use]
extern crate log;
extern crate ctrlc;
extern crate gnyprland_ui;
extern crate smol;

use std::{env, fs, mem, process, str::FromStr};

use clap::Parser;
use gnyprland_relay::message::{IpcMessage, IpcResponse};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use smol::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::unix::{UnixListener, UnixStream},
};
use time::macros::format_description;

#[cfg(debug_assertions)]
pub const LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
pub const LEVEL: LevelFilter = LevelFilter::Info;

#[derive(Clone, Debug, Parser)]
#[command(name = "gnyprland")]
#[command(version, about = "A Gnome-like Bar for Hyprland")]
pub struct Arguments {
    #[arg(short)]
    inspector: bool,
}

pub async fn receive(stream: &mut UnixStream) -> io::Result<String> {
    let mut length_buf = [0u8; mem::size_of::<usize>()];
    stream.read_exact(&mut length_buf).await?;
    let length = usize::from_be_bytes(length_buf);

    let mut message_buf = vec![0u8; length];
    stream.read_exact(&mut message_buf).await?;

    String::from_utf8(message_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub async fn send(stream: &mut UnixStream, message: String) -> io::Result<()> {
    stream.write_all(&message.len().to_be_bytes()).await?;
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}

async fn start_bar() {
    let (tx, rx) = gnyprland_relay::channel::<IpcMessage, IpcResponse>();

    let Ok(listener) = UnixListener::bind("/tmp/gnyprland.socket") else {
        error!("Failed to bind to socket");
        return;
    };

    smol::spawn(async move {
        let listener = listener;

        while let Ok((mut stream, _)) = listener.accept().await {
            debug!("Got new connection");

            let Ok(message) = receive(&mut stream).await else {
                error!("Failed to read message from stream");
                continue;
            };

            let Ok(message) = IpcMessage::from_str(message.trim()) else {
                error!("Failed to parse message: {message}");
                continue;
            };

            debug!("Got message: {message:?}");
            let Ok(res) = tx.send(message).await else {
                error!("Failed to process message");
                return;
            };

            debug!("Sending response: {res:?}");
            let Ok(_) = send(&mut stream, res.to_string()).await else {
                error!("Failed to write response to stream");
                return;
            };

            debug!("Dropping connection");
        }
    })
    .detach();

    ctrlc::set_handler(move || {
        info!("Received Ctrl+C, shutting down...");
        if let Err(e) = fs::remove_file("/tmp/gnyprland.socket") {
            error!("Failed to remove socket file: {e}");
        }
        process::exit(0);
    })
    .unwrap();

    info!("Starting UI");
    gnyprland_ui::start(rx);
}

async fn connect_to_socket() -> io::Result<UnixStream> {
    UnixStream::connect("/tmp/gnyprland.socket").await
}

fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_threads(cfg!(debug_assertions))
        .with_source_location(cfg!(debug_assertions))
        .with_local_timestamps()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .with_level(LEVEL)
        .init()
        .unwrap();

    let cli = Arguments::parse();

    if cli.inspector {
        smol::block_on(async {
            let Ok(mut stream) = connect_to_socket().await else {
                error!("Failed to connect to socket");
                return;
            };

            const MESSAGE: IpcMessage = IpcMessage::StartInspector;

            let Ok(_) = send(&mut stream, MESSAGE.to_string()).await else {
                error!("Failed to send message to socket");
                return;
            };

            debug!("Sent message: {MESSAGE:?}");

            let Ok(response) = receive(&mut stream).await else {
                error!("Failed to read message from socket");
                return;
            };

            println!("{response}");
        });

        return;
    }

    // run the bar
    // give smol some threads
    unsafe {
        env::set_var("SMOL_THREADS", "4");
    }

    smol::block_on(start_bar());
}

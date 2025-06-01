#![allow(clippy::missing_safety_doc)]

extern crate smol;
extern crate thiserror;

pub mod message;

use smol::channel;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RelayError {
    #[error("Failed to send message")]
    SendError,
    #[error("Failed to receive message")]
    ReceiveError,
}

#[derive(Debug)]
pub struct RelaySender<T, K> {
    tx: channel::Sender<T>,
    res_rx: channel::Receiver<K>,
}

#[derive(Debug)]
pub struct RelayResponder<K> {
    res_tx: channel::Sender<K>,
}

#[derive(Debug)]
pub struct RelayReceiver<T, K> {
    rx: channel::Receiver<T>,
    res_tx: channel::Sender<K>,
}

impl<T, K> RelaySender<T, K> {
    fn new(tx: channel::Sender<T>, res_rx: channel::Receiver<K>) -> Self {
        RelaySender { tx, res_rx }
    }

    pub async fn send(&self, value: T) -> Result<K, RelayError> {
        let Ok(_) = self.tx.send(value).await else {
            return Err(RelayError::SendError);
        };

        let Ok(message) = self.res_rx.recv().await else {
            return Err(RelayError::ReceiveError);
        };

        Ok(message)
    }
}

impl<K> RelayResponder<K> {
    fn new(res_tx: channel::Sender<K>) -> Self {
        RelayResponder { res_tx }
    }

    pub async fn respond(&self, value: K) -> Result<(), RelayError> {
        self.res_tx
            .send(value)
            .await
            .map_err(|_| RelayError::SendError)
    }
}

impl<T, K> RelayReceiver<T, K> {
    fn new(rx: channel::Receiver<T>, res_tx: channel::Sender<K>) -> Self {
        RelayReceiver { rx, res_tx }
    }

    pub async fn receive(&mut self) -> Result<T, RelayError> {
        let value = self.rx.recv().await.map_err(|_| RelayError::ReceiveError)?;
        Ok(value)
    }

    pub fn responder(&self) -> RelayResponder<K> {
        RelayResponder::new(self.res_tx.clone())
    }

    pub async fn respond(&self, value: K) -> Result<(), RelayError> {
        self.responder().respond(value).await
    }
}

pub fn channel<T, K>() -> (RelaySender<T, K>, RelayReceiver<T, K>) {
    let (tx, rx) = channel::unbounded();
    let (res_tx, res_rx) = channel::unbounded();
    (RelaySender::new(tx, res_rx), RelayReceiver::new(rx, res_tx))
}

use std::{any::Any, collections::HashMap, env, sync::Arc, thread};

use smol::{
    io::{AsyncBufReadExt, BufReader},
    net::unix::UnixStream,
};

use crate::{
    error::{EventParseError, ListenError},
    event::Event,
};

type AnyData = Arc<dyn Any + Send + Sync>;
type Callback = Arc<dyn Fn(AnyData) + Send + Sync>;

pub struct AnyEventStore {
    functions: Vec<Callback>,
    parser: fn(&[&str]) -> Result<AnyData, EventParseError>,
}

impl AnyEventStore {
    pub fn new<E: Event>() -> Self {
        let parser = |data: &[&str]| E::parse_data(data).map(|data| Arc::new(data) as AnyData);

        Self {
            functions: Vec::new(),
            parser,
        }
    }

    pub fn register(&mut self, f: Callback) {
        self.functions.push(f);
    }

    pub fn call(&self, data: &[&str]) -> Result<(), EventParseError> {
        let data = (self.parser)(data)?;

        for f in &self.functions {
            let data = Arc::clone(&data);
            let f = Arc::clone(f);
            thread::spawn(move || f(data));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct EventListener {
    events: HashMap<&'static str, AnyEventStore>,
}

impl EventListener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<E: Event>(&mut self, f: impl Fn(&E::Data) + Send + Sync + 'static) {
        let name = E::NAME;
        let store = self
            .events
            .entry(name)
            .or_insert_with(AnyEventStore::new::<E>);

        store.register(Arc::new(move |data| {
            let downcast = unsafe { data.downcast_unchecked::<E::Data>() };
            let data = downcast.as_ref();
            f(data);
        }));
    }

    pub fn listen(self) -> Result<!, ListenError> {
        let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
        let xrd = env::var("XDG_RUNTIME_DIR")?;
        let socket = format!("{xrd}/hypr/{his}/.socket2.sock");

        smol::block_on::<Result<!, ListenError>>(async move {
            let stream = UnixStream::connect(&socket).await?;
            let mut reader = BufReader::new(stream);

            loop {
                let mut buf = vec![];
                reader.read_until(b'\n', &mut buf).await?;

                let event = String::from_utf8(buf)?;
                let event = event.trim();
                let &[event, data] = event.split(">>").collect::<Vec<_>>().as_slice() else {
                    return Err(EventParseError::InvalidData)?;
                };

                let data_parts = data.split(',').map(str::trim).collect::<Vec<_>>();

                self.send(event, &data_parts)?;
            }
        })
    }

    fn send(&self, event: &str, data: &[&str]) -> Result<(), EventParseError> {
        if let Some(store) = self.events.get(event) {
            store.call(data)?;
        }

        Ok(())
    }
}

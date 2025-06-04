use chrono::{DateTime as DateTimeData, Local, Timelike};

use crate::{overlays::active::ActiveOverlay, prelude::*};

fn poll_datetime(mut callback: impl FnMut(DateTimeData<Local>) + Send + 'static) {
    thread::spawn(move || loop {
        let now = Local::now();
        callback(now);

        let seconds_remaining = 60 - now.second();
        let nanos_remaining = 1_000_000_000 - now.nanosecond();
        let sleep_duration = Duration::new(seconds_remaining as u64, nanos_remaining);
        thread::sleep(sleep_duration);
    });
}

#[derive(Clone, Debug)]
pub enum Message {
    DateTime(DateTimeData<Local>),
    Open(bool),
}

pub struct DateTime {
    time: DateTimeData<Local>,
    open: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for DateTime {
    type Init = ();
    type Input = Message;
    type Output = super::Message;

    view! {
        gtk::Button {
            #[watch]
            set_css_classes: css!["element", "open" if model.open],
            connect_clicked: move |_| {
                ActiveOverlay::set(Some(ActiveOverlay::Center))
            },


            gtk::Label {
                #[watch]
                set_label: &model.time.format("%A, %b %d  %l:%M %p").to_string(),
                #[watch]
                set_css_classes: css!["text text-lg"],
            },
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = DateTime {
            time: Local::now(),
            open: false,
        };

        ActiveOverlay::on_change(sender.input_sender(), |open| {
            Message::Open(open == Some(ActiveOverlay::Center))
        });

        poll_datetime(clone!(
            #[strong]
            sender,
            move |dt| {
                sender.input(Message::DateTime(dt));
            }
        ));

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            Message::DateTime(dt) => {
                self.time = dt;
            }
            Message::Open(open) => {
                self.open = open;
            }
        }
    }
}

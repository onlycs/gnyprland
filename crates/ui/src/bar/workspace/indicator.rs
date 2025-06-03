use hyprland::{
    command::{Executor, Workspaces},
    event,
};

use crate::prelude::*;

fn calculate() -> u16 {
    Executor::command::<Workspaces>()
        .unwrap()
        .into_iter()
        .take(10)
        .map(|n| (str::parse(&n.name).unwrap_or(1) - 1, n.windows))
        .filter(|(_, n)| *n > 0)
        .fold(0u16, |mask, (i, _)| mask | (1 << i))
}

fn cname(mask: u16, n: usize) -> Vec<&'static str> {
    css!["workspace-indicator", "with-windows" if mask & (1 << n) > 0].to_vec()
}

pub struct OpenIndicator {
    mask: u16,
}

pub struct IndicatorWidgets {
    indicators: Vec<gtk::Box>,
}

impl SimpleComponent for OpenIndicator {
    type Init = ();
    type Input = u16;
    type Output = ();
    type Root = gtk::Box;
    type Widgets = IndicatorWidgets;

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        thread::spawn(move || {
            let mut listener = EventListener::new();

            listener.register::<event::OpenWindow>(clone!(
                #[strong]
                sender,
                move |_| sender.input(calculate())
            ));

            listener.register::<event::CloseWindow>(clone!(
                #[strong]
                sender,
                move |_| sender.input(calculate())
            ));

            listener.register::<event::MoveWindow>(clone!(
                #[strong]
                sender,
                move |_| sender.input(calculate())
            ));

            debug!("Watching for window changes");
            listener.listen().unwrap();
        });

        let mask = calculate();

        let mut indicators = vec![];
        for i in 0..10 {
            let indicator = gtk::Box::builder().css_classes(cname(mask, i)).build();

            root.append(&indicator);
            indicators.push(indicator);
        }

        let model = OpenIndicator { mask: 0 };
        let widgets = IndicatorWidgets { indicators };

        ComponentParts { model, widgets }
    }

    fn init_root() -> Self::Root {
        gtk::Box::builder().spacing(6).build()
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        self.mask = message;
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _: ComponentSender<Self>) {
        for (i, w) in widgets.indicators.iter_mut().enumerate() {
            w.set_css_classes(cname(self.mask, i).as_slice());
        }
    }
}

mod window;

use notify::{recommended_watcher, Watcher};
use relm4::gtk::{gdk::Display, CssProvider};
use window::ActiveWindow;

use crate::{css, prelude::*};

const HEIGHT: i32 = 48;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OpenMenu {
    Calendar,
    QuickSettings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Message {
    Open(OpenMenu),
    CloseMenu,
    ReloadCSS,
}

struct Bar {
    open: Option<OpenMenu>,
    css: gtk::CssProvider,

    active_window: Controller<ActiveWindow>,
}

#[relm4::component]
impl SimpleComponent for Bar {
    type Init = ();
    type Input = Message;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_title: Some("panel"),
            set_default_height: HEIGHT,

            init_layer_shell: (),
            set_layer: Layer::Top,
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),
            set_exclusive_zone: HEIGHT,

            set_hexpand: true,
            set_css_classes: &["Bar"],

            gtk::Box {
                set_spacing: 8,

                #[local_ref]
                active_window -> gtk::Box {}
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        cfg_if! {
            if #[cfg(debug_assertions)] {
                css::begin_watch(move || sender.input_sender().emit(Message::ReloadCSS));
            } else {
                css::write_css().unwrap();
                sender.input_sender().emit(Message::ReloadCSS);
            }
        }

        let active_window_ctl = ActiveWindow::builder().launch(()).detach();
        let active_window = active_window_ctl.widget();

        // setup return values
        let css = CssProvider::new();
        let widgets = view_output!();
        let model = Bar {
            open: None,
            css,
            active_window: active_window_ctl,
        };

        // register css provider
        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &model.css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        match message {
            Message::Open(_) => {}
            Message::CloseMenu => {}
            Message::ReloadCSS => {
                debug!("Reloading CSS");
                self.css.load_from_path(css::FILE);
            }
        }
    }
}

pub fn run() {
    RelmApp::new("page.angad.gnyprland").run::<Bar>(());
}

mod datetime;
mod window;
mod workspace;

use datetime::DateTime;
use gnyprland_relay::{
    message::{IpcMessage, IpcReceiver, IpcResponse},
    RelayResponder,
};
use relm4::gtk::{gdk::Display, CssProvider};
use window::ActiveWindow;
use workspace::ActiveWorkspace;

use crate::{css, overlays, prelude::*};

const HEIGHT: i32 = 57;

#[derive(Clone, Debug)]
pub enum Message {
    Ipc(IpcMessage),
    ReloadCSS,
}

#[allow(dead_code)]
pub struct Bar {
    responder: RelayResponder<IpcResponse>,
    css: gtk::CssProvider,

    active_window: Controller<ActiveWindow>,
    active_workspace: Controller<ActiveWorkspace>,
    datetime: Controller<DateTime>,
}

#[relm4::component(pub)]
impl SimpleComponent for Bar {
    type Init = IpcReceiver;
    type Input = Message;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_title: Some("panel"),
            set_default_height: HEIGHT,

            init_layer_shell: (),
            set_layer: Layer::Bottom,
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Left, true),
            set_exclusive_zone: HEIGHT,

            set_hexpand: true,
            set_css_classes: &["bar"],

            gtk::CenterBox {
                #[wrap(Some)]
                set_start_widget = &gtk::Box {
                    set_spacing: 8,

                    #[local_ref]
                    active_window_widget -> root!(ActiveWindow),

                    #[local_ref]
                    active_workspace_widget -> root!(ActiveWorkspace),
                },

                #[local_ref]
                #[wrap(Some)]
                set_center_widget = datetime_widget -> root!(DateTime),
            }
        }
    }

    fn init(
        mut init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        cfg_if! {
            if #[cfg(debug_assertions)] {
                debug!("Watching for CSS changes");
                let sender_clone = sender.clone();
                css::begin_watch(move || sender_clone.input_sender().emit(Message::ReloadCSS));
            } else {
                css::write_css().unwrap();
                sender.input_sender().emit(Message::ReloadCSS);
            }
        }

        let active_window = ActiveWindow::builder().launch(()).detach();

        let active_workspace = ActiveWorkspace::builder().launch(()).detach();
        let datetime = DateTime::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let active_window_widget = active_window.widget();
        let active_workspace_widget = active_workspace.widget();
        let datetime_widget = datetime.widget();

        // setup return values
        let css = CssProvider::new();
        let widgets = view_output!();
        let model = Bar {
            responder: init.responder(),
            css,
            active_window,
            active_workspace,
            datetime,
        };

        // register css provider
        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &model.css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // forward messages
        smol::spawn(async move {
            while let Ok(message) = init.receive().await {
                sender.input_sender().emit(Message::Ipc(message));
            }
        })
        .detach();

        overlays::attach_windows();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        match message {
            Message::Ipc(ipc) => {
                match ipc {
                    IpcMessage::StartInspector => {
                        cfg_if! {
                            if #[cfg(debug_assertions)] {
                                debug!("Toggling inspector");
                                gtk::Window::set_interactive_debugging(true);
                            } else {
                                warn!("Inspector is only available in debug mode");
                                smol::block_on(self.responder.respond(IpcResponse::InspectorNotAvailable)).unwrap();
                                return;
                            }
                        }
                    }
                    IpcMessage::ReloadCSS => {
                        debug!("Reloading CSS");
                        self.css.load_from_path(css::FILE);
                    }
                };

                smol::block_on(self.responder.respond(IpcResponse::Ok)).unwrap();
            }
            Message::ReloadCSS => {
                debug!("Reloading CSS");
                self.css.load_from_path(css::FILE);
            }
        }
    }
}

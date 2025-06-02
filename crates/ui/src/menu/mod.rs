use relm4::{gtk::prelude::GtkApplicationExt, Component, ComponentController};

pub mod open;
pub mod overlay;

pub fn attach_windows() {
    let app = relm4::main_application();

    let overlay = overlay::Overlay::builder();

    app.add_window(&overlay.root);

    overlay.launch(()).detach_runtime();
}

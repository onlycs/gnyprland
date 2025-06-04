use clickoff::ClickOff;
use relm4::{
    gtk::{self, glib::object::IsA, prelude::GtkApplicationExt, Application},
    Component, ComponentController,
};

use super::center_menu::CenterMenu;

pub mod active;
mod clickoff;

fn attach_window<C: Component<Init = ()>>(app: &Application)
where
    <C as Component>::Root: IsA<gtk::Window>,
{
    let component = C::builder();
    app.add_window(&component.root);
    component.launch(()).detach_runtime();
}

pub fn attach_windows() {
    let app = relm4::main_application();

    attach_window::<CenterMenu>(&app);
    attach_window::<ClickOff>(&app);
}

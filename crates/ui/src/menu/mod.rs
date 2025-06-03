use center::CenterMenu;
use overlay::Overlay;
use relm4::{
    gtk::{self, glib::object::IsA, prelude::GtkApplicationExt, Application},
    Component, ComponentController,
};

mod center;
pub mod open;
mod overlay;

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
    attach_window::<Overlay>(&app);
}

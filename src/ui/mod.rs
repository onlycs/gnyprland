use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

static mut APP: Option<&'static Application> = None;

mod bar;
mod components;

pub mod prelude {
    pub use super::components::*;
    pub use crate::ui;
}

pub fn run_blocking() {
    gtk::init().unwrap();

    let app = Box::leak(Box::new(Application::new()));

    app.connect_activate(|_app| {
        let window = bar::bar();

        _app.add_window(&window);
        window.show_all();

        unsafe { APP = Some(app) }
    });

    app.acquire_socket().unwrap();

    app.run();
}

#[allow(static_mut_refs)]
pub async fn get_app() -> &'static Application {
    unsafe {
        while APP.is_none() {
            async_std::task::yield_now().await;
        }

        APP.unwrap()
    }
}

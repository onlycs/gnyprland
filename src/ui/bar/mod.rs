mod datetime;
mod window;
mod workspace;

use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

pub fn bar() -> Window {
    render! {
        Window {
            name: "bar",
            anchor: WindowAnchor::TOP | WindowAnchor::LEFT | WindowAnchor::RIGHT,
            exclusivity: Exclusivity::Exclusive,
            child: opt CenterBox {
                child start_widget: bor AstalBox {
                    spacing: 8,
                    children {
                        inh fun window::ActiveWindow,
                        inh fun workspace::Workspace
                    }
                },
                class_name: ["Bar"],
                with: |widget| {
                    CenterBoxExt::set_center_widget(widget, &datetime::DateTime::widget());
                },
            },
            monitor: 0,
        }
    }
}

mod window;

use crate::prelude::*;

pub fn bar() -> Window {
    widget! {
        Window {
            name: "bar",
            anchor: WindowAnchor::Top + WindowAnchor::Right + WindowAnchor::Left,
            exclusivity: Exclusivity::Exclusive,
            child: opt CenterBox {
                child start_widget: bor AstalBox {
                    spacing: 8,
                    children {
                        inh fun() window::ActiveWindow {}
                    }
                }
            },
            class_name: ["Bar"],
        }
    }
}

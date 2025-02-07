use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

#[widget]
pub fn DateTime() -> Widget {
    render! {
        inh fun Interactable {
            child: inh AstalBox {
                children {
                    inh Label {
                        bind label: variables::date().bind(),
                        class_name: ["TextMain", "TextLarge", "DateTime"],
                        hexpand: true,
                        xalign: 1.0,
                    },
                    inh Label {
                        label: " — ",
                        class_name: ["TextMain", "TextLarge", "DateTime"],
                    },
                    inh Label {
                        bind label: variables::time().bind(),
                        class_name: ["TextMain", "TextLarge", "DateTime"],
                        hexpand: true,
                        xalign: 0.0,
                    }
                },
                spacing: 4,
                class_name: ["BarElement", "DateTimeBox"]
            }
        }
    }
}

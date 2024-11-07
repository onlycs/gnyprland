use astal::{
    traits::{BoxExt, CenterBoxExt, WindowExt},
    Box, CenterBox, Exclusivity, Label, Window, WindowAnchor,
};
use gtk::traits::{
    BoxExt as GtkBoxExt, ContainerExt, LabelExt as GtkLabelExt, StyleContextExt, WidgetExt,
};
use widget::widget;

pub fn bar() -> Window {
    widget! {
        Window {
            widget_name: "bar",
            anchor: WindowAnchor::Top + WindowAnchor::Right + WindowAnchor::Left,
            exclusivity: Exclusivity::Exclusive,
            child: CenterBox {
                child start_widget: bor Box {
                    spacing: 8,
                    children {
                        Label {
                            text: "Hello, World!",
                            class_name: ["Bar"],
                        }
                    }
                }
            },
            class_name: ["Bar"],
        }
    }
}

pub use astal::{
    traits::{ApplicationExt, BoxExt, CenterBoxExt, EventBoxExt, WindowExt},
    Application, Box as AstalBox, CenterBox, EventBox, Exclusivity, Label, Window, WindowAnchor,
};

pub use gtk::traits::{
    BoxExt as GtkBoxExt, ContainerExt as GtkContainerExt, EventBoxExt as GtkEventBoxExt,
    LabelExt as GtkLabelExt, StyleContextExt as GtkStyleContextExt, WidgetExt as GtkWidgetExt,
};

pub use crate::ui::prelude::*;
pub use widget::widget;

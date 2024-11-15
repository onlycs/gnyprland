pub use astal::{
    traits::{ApplicationExt, BoxExt, CenterBoxExt, EventBoxExt, LabelExt, WindowExt},
    Application, Box as AstalBox, CenterBox, EventBox, Exclusivity, Label, Window, WindowAnchor,
};

pub use astal_hyprland::{traits::*, Bind as HyprlandBind, Client, Hyprland, Workspace};
pub use astal_io::{traits::VariableExt, Variable as AstalVariable};

pub use gtk::{
    glib::ToValue,
    traits::{
        BoxExt as GtkBoxExt, ContainerExt as GtkContainerExt, EventBoxExt as GtkEventBoxExt,
        GtkApplicationExt, GtkWindowExt, LabelExt as GtkLabelExt,
        StyleContextExt as GtkStyleContextExt, WidgetExt as GtkWidgetExt,
    },
};

pub use gio::prelude::{
    ApplicationExt as GioApplicationExt, ApplicationExtManual as GioApplicationExtManual,
};

pub use glib::Value;

pub use crate::binding::*;
pub use crate::services;
pub use crate::ui::prelude::*;
pub use widget::widget;

pub fn forever<T>(obj: T) -> &'static T {
    Box::leak(Box::new(obj))
}

pub fn forever_mut<T>(obj: T) -> &'static mut T {
    Box::leak(Box::new(obj))
}

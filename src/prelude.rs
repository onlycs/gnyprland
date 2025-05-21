pub mod astal_obj {
    pub use astal::{
        traits::{ApplicationExt, BoxExt, CenterBoxExt, EventBoxExt, LabelExt, WindowExt},
        Application, Box as AstalBox, CenterBox, EventBox, Exclusivity, Label, Window,
        WindowAnchor,
    };
    pub use astal_hyprland::{traits::*, Bind as HyprlandBind, Client, Hyprland, Workspace};
    pub use astal_io::{
        traits::{ApplicationExt as IOApplicationExt, VariableExt},
        Variable as AstalVariable,
    };
}

pub mod gtk_obj {
    pub use gio::prelude::{
        ApplicationExt as GioApplicationExt, ApplicationExtManual as GioApplicationExtManual,
    };
    pub use gtk::{
        glib::ToValue,
        traits::{
            BoxExt as GtkBoxExt, ButtonExt as GtkButtonExt, ContainerExt as GtkContainerExt,
            EventBoxExt as GtkEventBoxExt, GtkApplicationExt, GtkWindowExt,
            LabelExt as GtkLabelExt, StyleContextExt as GtkStyleContextExt,
            WidgetExt as GtkWidgetExt,
        },
    };
}

pub use glib::Value;

pub use crate::binding::*;
pub use crate::ggc;
pub use crate::services;
pub use crate::ui::prelude::*;
pub use crate::variables;
pub use macros::*;

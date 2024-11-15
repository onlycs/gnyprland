use crate::prelude::*;

macro_rules! service {
    ($init:expr, $ty:ty, $name:ident) => {
        #[allow(static_mut_refs)]
        mod $name {
            use super::*;
            static mut __SERVICE: Option<$ty> = None;

            pub fn $name() -> &'static $ty {
                unsafe { __SERVICE.get_or_insert_with(|| $init) }
            }
        }

        pub use $name::$name;
    };
}

service!(Hyprland::default().unwrap(), Hyprland, hyprland);

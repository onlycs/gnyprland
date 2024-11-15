use std::sync::Arc;

use async_std::sync::Mutex;
use glib::Pointer;
use gtk::glib::ObjectExt;

use crate::prelude::*;

pub struct ActiveWorkspaceProps {
    activeid: Binding<'static, Hyprland, u32>,
}

pub struct OpenWindowsProps {
    bitmask: Binding<'static, Variable, u32>,
}

#[allow(non_snake_case)]
pub fn ActiveWorkspace(ActiveWorkspaceProps { activeid }: ActiveWorkspaceProps) -> AstalBox {
    const W_DOT: u32 = 6;
    const W_SPACE: u32 = 6;

    let class_before = |id: u32| -> String {
        match id {
            0 => String::from("background-color: transparent;"),
            _ => format!("min-width: {}px;", (id * W_DOT) + ((id - 1) * W_SPACE)),
        }
    };

    let class_current = |id: u32| -> String {
        let margin = match id {
            0 => "margin-left: 0",
            9 => "margin-right: 0",
            _ => "",
        };

        format!("min-width: 6px; {margin};")
    };

    let class_after = |id: u32| -> String {
        match id {
            9 => String::from("background-color: transparent;"),
            _ => format!(
                "min-width: {}px;",
                ((9 - id) * W_DOT) + ((8 - id) * W_SPACE)
            ),
        }
    };

    widget! {
        AstalBox {
            class_name: ["SliderBox"],
            children {
                AstalBox {
                    class_name: ["SliderSegment"],
                    bind css: activeid.transform(class_before),
                },
                AstalBox {
                    class_name: ["SliderSegment", "Current"],
                    bind css: activeid.transform(class_current),
                },
                AstalBox {
                    class_name: ["SliderSegment"],
                    bind css: activeid.transform(class_after),
                },
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn OpenWindows(OpenWindowsProps { bitmask }: OpenWindowsProps) -> AstalBox {
    let class_of = |bitmask: u32, wksp: u32| {
        let mut classname = vec!["WorkspaceIndicator"];

        if wksp == 9 {
            classname.push("Last")
        }

        if bitmask & 1 << wksp != 0 {
            classname.push("Windows");
        }

        println!("{:?} {wksp}", classname);

        classname
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    };

    let bitmasks = (0..10)
        .into_iter()
        .map(|id| move |mask| class_of(mask, id))
        .collect::<Vec<_>>();

    widget! {
        AstalBox {
            children {
                // AstalBox { apply class_name: bitmask.transform(bitmasks[0]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[1]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[2]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[3]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[4]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[5]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[6]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[7]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[8]) },
                // AstalBox { apply class_name: bitmask.transform(bitmasks[9]) },
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn Workspace() -> EventBox {
    let hyprland = services::hyprland();

    widget! {
        fun(interactable::Props) Interactable {
            child: AstalBox {
                class_name: ["BarElement", "WorkspaceBox"],
                vertical: true,
                homogeneous: true,
                children {
                    inh fun(ActiveWorkspaceProps) ActiveWorkspace {
                        activeid: hyprland.bind::<Workspace>("focused_workspace")
                            .transform(|wksp| wksp.id() - 1)
                            .transform(|id| id as u32),
                    },
                }
            }
        }
    }
}

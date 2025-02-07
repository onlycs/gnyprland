use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

#[widget]
pub fn ActiveWorkspace(activeid: Binding<'static, Hyprland, u32>) -> Widget {
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

    render! {
        inh AstalBox {
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

#[widget]
pub fn OpenWindows(bitmask: Binding<'static, Variable<u32>, u32>) -> Widget {
    let class_of = |bitmask: u32, wksp: u32| {
        let mut classname = vec!["WorkspaceIndicator"];

        if wksp == 9 {
            classname.push("Last")
        }

        if bitmask & 1 << wksp != 0 {
            classname.push("Windows");
        }

        classname
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>()
    };

    let bitmasks = (0..10)
        .into_iter()
        .map(|id| move |mask| class_of(mask, id))
        .collect::<Vec<_>>();

    let boxes = (0..10)
        .into_iter()
        .map(|id| render! { inh AstalBox { bind class_name: bitmask.transform(bitmasks[id]) } })
        .collect::<Vec<_>>();

    render! {
        inh AstalBox {
            children: boxes.as_slice()
        }
    }
}

#[widget]
pub fn Workspace() -> Widget {
    let hyprland = services::hyprland();
    let bitmask = forever(Variable::new(0u32));

    hyprland.connect_event(|hypr, _, _| {
        let mask = hypr.workspaces().iter().fold(0u32, |mask, wk| {
            mask | (wk.clients().len().min(1) << wk.id().max(1) - 1) as u32
        });

        bitmask.set(mask);
    });

    render! {
        inh fun Interactable {
            child: inh AstalBox {
                class_name: ["BarElement", "WorkspaceBox"],
                vertical: true,
                homogeneous: true,
                children {
                    inh fun ActiveWorkspace {
                        activeid: hyprland.bind::<astal_hyprland::Workspace>("focused_workspace")
                            .transform(|wksp| wksp.id() - 1)
                            .transform(|id| id as u32),
                    },
                    inh fun OpenWindows {
                        bitmask: bitmask.bind(),
                    }
                }
            }
        }
    }
}

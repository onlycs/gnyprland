use glib::GString;

use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

fn get_title_transformer(class: &String) -> fn(String) -> String {
    match class.as_str() {
        "kitty" => |_| "Terminal".to_string(),
        "code-url-handler" => |s| s.replace("Visual Studio Code", "VSCode"),
        "firefox" => |s| s.replace("Mozilla Firefox", "Firefox"),
        "" => |_| "Desktop".to_string(),
        _ => |s| s,
    }
}

fn override_class(class: String) -> String {
    match class.as_str() {
        "code-url-handler" => "vscode".to_string(),
        "dev.zed.Zed" => "zed".to_string(),
        "" => "hyprland".to_string(),
        _ => class.clone(),
    }
}

fn override_title(title: String, class: &String) -> String {
    get_title_transformer(class)(title)
}

#[widget]
pub fn ActiveWindow() -> Widget {
    let hyprland = services::hyprland();

    render! {
        inh fun Interactable {
            child: inh AstalBox {
                children {
                    Label {
                        bind label: hyprland.bind::<Client>("focused_client").transform(|client|
                            override_title(
                                client.title().as_ref().map(GString::to_string).unwrap_or_default(),
                                &ClientExt::class(&client).as_ref().map(GString::to_string).unwrap_or_default(),
                            )
                        ).or("Desktop"),
                        class_name: ["TextMain"],
                        max_width_chars: 10,
                        truncate: true,
                    },
                    Label {
                        bind label: hyprland.bind::<Client>("focused_client").transform(|client|
                            override_class(
                                ClientExt::class(&client).as_ref().map(GString::to_string).unwrap_or_default(),
                            )
                        ).or("hyprland"),
                        class_name: ["TextSub"],
                        max_width_chars: 10,
                        truncate: true,
                    },
                },
                class_name: ["BarElement", "ActiveWindow"],
                vertical: true,
            }
        }
    }
}

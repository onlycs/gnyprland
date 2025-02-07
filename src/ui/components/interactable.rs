use crate::consts;
use crate::prelude::*;
use astal_obj::*;
use gtk_obj::*;

#[widget]
pub fn Interactable(child: gtk::Widget) -> EventBox {
    let event_box = EventBox::new();
    event_box.set_child(Some(&child));

    event_box.connect_hover(|b, _| {
        b.children()[0]
            .style_context()
            .add_class(consts::HOVER_CLASS)
    });

    event_box.connect_hover_lost(|b, _| {
        b.children()[0]
            .style_context()
            .remove_class(consts::HOVER_CLASS)
    });

    event_box.connect_click(|b, _| {
        b.children()[0]
            .style_context()
            .add_class(consts::ACTIVE_CLASS)
    });

    event_box.connect_click_release(|b, _| {
        b.children()[0]
            .style_context()
            .remove_class(consts::ACTIVE_CLASS)
    });

    event_box.into()
}

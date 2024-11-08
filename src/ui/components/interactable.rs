use gtk::glib::IsA;

use crate::consts;
use crate::prelude::*;

#[derive(Default)]
pub struct Props<'a, W: IsA<gtk::Widget>> {
    pub child: Option<&'a W>,
}

#[allow(non_snake_case)]
pub fn Interactable<'a, W: IsA<gtk::Widget>>(props: Props<'a, W>) -> EventBox {
    let event_box = EventBox::new();
    event_box.set_child(props.child);

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

    event_box
}

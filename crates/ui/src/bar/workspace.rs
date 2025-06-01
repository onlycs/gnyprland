mod active;
mod open;

use active::ActiveSlider;
use open::OpenIndicator;

use crate::prelude::*;

#[allow(dead_code)]
pub struct ActiveWorkspace {
    slider: Controller<ActiveSlider>,
    open: Controller<OpenIndicator>,
}

#[relm4::component(pub)]
impl SimpleComponent for ActiveWorkspace {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: relm4::gtk::Orientation::Vertical,
            set_css_classes: &["BarElement", "ActiveWorkspace"],

            #[local_ref]
            slider_widget -> <ActiveSlider as Component>::Root {},

            #[local_ref]
            open_widget -> <OpenIndicator as Component>::Root {},
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let slider = ActiveSlider::builder().launch(()).detach();
        let open = OpenIndicator::builder().launch(()).detach();

        let slider_widget = slider.widget();
        let open_widget = open.widget();

        let widgets = view_output!();
        let model = ActiveWorkspace { slider, open };

        ComponentParts { model, widgets }
    }
}

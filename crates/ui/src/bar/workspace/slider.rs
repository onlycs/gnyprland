use std::{
    f64,
    sync::{
        atomic::{self, AtomicPtr, AtomicU8},
        Arc, RwLock,
    },
};

use hyprland::{data::Workspace, shared::HyprDataActive};
use relm4::gtk::{
    glib::{timeout_add, translate::FromGlibPtrNone, ControlFlow},
    DrawingArea,
};

use crate::prelude::*;

const ANIM_DURATION: f64 = 0.1;

fn translate(a: f64, b: f64, t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t) * (b - a) + a
}

fn recalculate_active() -> u8 {
    let active = Workspace::get_active().unwrap();

    trace!("New active workspace: {}", active.name);
    str::parse(&active.name).unwrap()
}

pub struct DrawData {
    last: AtomicU8,
    current: AtomicU8,
    switch: RwLock<Instant>,
}

pub struct ActiveSlider {
    draw_data: Arc<DrawData>,
}

pub struct ActiveWorkspaceWidgets {
    root: gtk::DrawingArea,
}

impl SimpleComponent for ActiveSlider {
    type Init = ();
    type Input = u8;
    type Output = ();
    type Root = gtk::DrawingArea;
    type Widgets = ActiveWorkspaceWidgets;

    fn init_root() -> Self::Root {
        gtk::DrawingArea::builder().css_classes(["slider"]).build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current = recalculate_active();
        let draw_data = Arc::new(DrawData {
            last: AtomicU8::new(current - 1),
            current: AtomicU8::new(current - 1),
            switch: RwLock::new(Instant::now()),
        });
        let widgets = ActiveWorkspaceWidgets { root };
        let model = ActiveSlider {
            draw_data: Arc::clone(&draw_data),
        };

        thread::spawn(move || {
            let mut listener = EventListener::new();

            listener.add_workspace_changed_handler(clone!(
                #[strong]
                sender,
                move |_| sender.input(recalculate_active()),
            ));

            debug!("Watching for active workspace changes");
            listener.start_listener().unwrap()
        });

        widgets.root.set_draw_func(move |_, ctx, _w, _h| {
            // step 0. reuse
            let wksp_to_draw = |position: f64| 3.0 + (position * 12.0);

            // step 1. calculate the position of the dot
            let last = draw_data.last.load(atomic::Ordering::Relaxed);
            let current = draw_data.current.load(atomic::Ordering::Relaxed);
            let elapsed = draw_data.switch.read().unwrap().elapsed().as_secs_f64();
            let dot_pos = translate(last as f64, current as f64, elapsed / ANIM_DURATION);
            let dot_pos_px = wksp_to_draw(dot_pos);

            // step 2. draw a dot at the calculated position
            ctx.arc(dot_pos_px, 3.0, 3.0, 0.0, f64::consts::TAU);
            ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
            ctx.fill().unwrap();

            // step 3. draw a dot to the left and the right of the current position
            let prev = wksp_to_draw(dot_pos - 1.0);
            let next = wksp_to_draw(dot_pos + 1.0);

            ctx.arc(prev, 3.0, 3.0, 0.0, f64::consts::TAU);
            ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
            ctx.fill().unwrap();

            ctx.arc(next, 3.0, 3.0, 0.0, f64::consts::TAU);
            ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
            ctx.fill().unwrap();

            // step 4. draw a dot at the ends, and fill in gaps
            let first = wksp_to_draw(0.0);
            let last = wksp_to_draw(9.0);

            if dot_pos > 1.0 {
                ctx.arc(first, 3.0, 3.0, 0.0, f64::consts::TAU);
                ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                ctx.fill().unwrap();

                ctx.rectangle(first, 0.0, prev - first, 6.0);
                ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                ctx.fill().unwrap();
            }

            if dot_pos < 8.0 {
                ctx.arc(last, 3.0, 3.0, 0.0, f64::consts::TAU);
                ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                ctx.fill().unwrap();

                ctx.rectangle(next, 0.0, last - next, 6.0);
                ctx.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                ctx.fill().unwrap();
            }
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        *self.draw_data.switch.write().unwrap() = Instant::now();
        self.draw_data.last.store(
            self.draw_data.current.load(atomic::Ordering::Relaxed),
            atomic::Ordering::Relaxed,
        );
        self.draw_data
            .current
            .store(message - 1, atomic::Ordering::Relaxed);
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _: ComponentSender<Self>) {
        let area = widgets.root.clone();
        let draw_data = Arc::clone(&self.draw_data);
        let last_drawing = *draw_data.switch.read().unwrap();
        let area_ptr = AtomicPtr::new(area.as_ptr());

        timeout_add(Duration::from_millis(5), move || {
            // safety: I shouldn't have to do this, fuck you.
            let ptr = area_ptr.load(atomic::Ordering::Relaxed);
            let area = unsafe { DrawingArea::from_glib_none(ptr) };

            area.queue_draw();

            if last_drawing.elapsed().as_secs_f64() > ANIM_DURATION {
                ControlFlow::Break
            } else {
                ControlFlow::Continue
            }
        });
    }
}

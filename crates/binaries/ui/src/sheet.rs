//! A sheet that pans and zooms. The trial loads a fixed SVG of 5,000+ elements; the pan and zoom
//! state lives in signals, so every move goes through Dioxus's own event path.
//!
//! Panning moves the drawn layer, which is cheap. Zooming changes the drawing's width instead of
//! scaling the layer: a scaled layer is a stretched bitmap and goes blurry, while a resized SVG is
//! redrawn sharp.

use dioxus::prelude::*;

const TRIAL_SHEET: &str = include_str!("../assets/trial/sheet.svg");
const STEP: f64 = 1.25;
const NUDGE: f64 = 40.0;
const SMALLEST: f64 = 0.1;
const LARGEST: f64 = 20.0;

/// How the sheet is viewed. It's shared, so the menus and shortcuts can zoom the sheet too.
#[derive(Clone, Copy, PartialEq)]
pub struct View {
    zoom: Signal<f64>,
    offset: Signal<(f64, f64)>,
    size: Signal<(f64, f64)>,
}

impl View {
    pub fn new() -> Self {
        Self {
            zoom: Signal::new(1.0),
            offset: Signal::new((0.0, 0.0)),
            size: Signal::new((0.0, 0.0)),
        }
    }

    pub fn zoom_in(self) {
        self.zoom_at(STEP, self.centre());
    }

    pub fn zoom_out(self) {
        self.zoom_at(1.0 / STEP, self.centre());
    }

    pub fn reset(mut self) {
        self.zoom.set(1.0);
        self.offset.set((0.0, 0.0));
    }

    fn pan_by(mut self, dx: f64, dy: f64) {
        let (x, y) = *self.offset.read();
        self.offset.set((x + dx, y + dy));
    }

    /// Zooms by `factor`, keeping the point `at` (from the sheet area's corner) where it is.
    fn zoom_at(mut self, factor: f64, at: (f64, f64)) {
        let old = *self.zoom.read();
        let new = (old * factor).clamp(SMALLEST, LARGEST);
        let kept = new / old;
        let (x, y) = *self.offset.read();
        self.zoom.set(new);
        self.offset
            .set((at.0 - (at.0 - x) * kept, at.1 - (at.1 - y) * kept));
    }

    fn centre(&self) -> (f64, f64) {
        let (width, height) = *self.size.read();
        (width / 2.0, height / 2.0)
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

#[component]
pub fn Sheet() -> Element {
    let mut view = use_context::<View>();
    let mut grab = use_signal(|| None::<(f64, f64)>);
    let (x, y) = *view.offset.read();
    let zoom = *view.zoom.read();
    let percent = (zoom * 100.0).round();
    let width = zoom * 100.0;

    rsx! {
        div {
            class: "sheet",
            tabindex: 0,
            role: "group",
            aria_label: "Sheet E-101, drive bus and control wiring",
            aria_describedby: "sheet-keys",
            onresize: move |evt| {
                if let Ok(size) = evt.get_border_box_size() {
                    view.size.set((size.width, size.height));
                }
            },
            onkeydown: move |evt| {
                match evt.key() {
                    Key::ArrowLeft => view.pan_by(NUDGE, 0.0),
                    Key::ArrowRight => view.pan_by(-NUDGE, 0.0),
                    Key::ArrowUp => view.pan_by(0.0, NUDGE),
                    Key::ArrowDown => view.pan_by(0.0, -NUDGE),
                    Key::Character(c) if c == "+" || c == "=" => view.zoom_in(),
                    Key::Character(c) if c == "-" => view.zoom_out(),
                    Key::Character(c) if c == "0" => view.reset(),
                    _ => return,
                }
                evt.prevent_default();
            },
            // The drawing ignores the pointer (app.css), so these coordinates are the sheet area's.
            onwheel: move |evt| {
                evt.prevent_default();
                let factor = if evt.delta().strip_units().y < 0.0 { STEP } else { 1.0 / STEP };
                let at = evt.element_coordinates();
                view.zoom_at(factor, (at.x, at.y));
            },
            onmousedown: move |evt| {
                let at = evt.client_coordinates();
                grab.set(Some((at.x, at.y)));
            },
            onmousemove: move |evt| {
                let Some((from_x, from_y)) = grab() else { return };
                let at = evt.client_coordinates();
                view.pan_by(at.x - from_x, at.y - from_y);
                grab.set(Some((at.x, at.y)));
            },
            onmouseup: move |_| grab.set(None),
            onmouseleave: move |_| grab.set(None),
            div {
                class: "canvas",
                style: "transform: translate({x}px, {y}px); width: {width}%",
                SheetArt {}
            }
        }
        p { id: "sheet-keys", class: "hint",
            "Arrow keys pan, plus and minus zoom, and 0 fits the sheet. Zoom {percent}%."
        }
    }
}

/// The drawing itself. It takes no props, so moving the view never re-renders it.
#[component]
fn SheetArt() -> Element {
    rsx! {
        div { class: "art", dangerous_inner_html: TRIAL_SHEET }
    }
}

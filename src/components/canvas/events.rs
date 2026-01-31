use super::utils::*;
use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;
use dioxus_elements::input_data::MouseButton;

// ========= Handlers principaux ========= //

pub fn handle_resize(mut canvas_size: Signal<CanvasSize>) -> impl FnMut(Event<ResizeData>) {
    move |data| {
        if let Ok(size) = data.get_border_box_size() {
            let new_size = CanvasSize::from((size.width as f64, size.height as f64));
            if *canvas_size.read() != new_size {
                canvas_size.set(new_size);
            }
        }
    }
}

pub fn handle_mouse_move(
    mut mouse: Signal<MousePos>,
    mut last_mouse: Signal<MousePos>,
    pan: Signal<Pan>,
    is_panning: Signal<bool>,
) -> impl FnMut(Event<MouseData>) {
    move |event: Event<MouseData>| {
        let coords = event.data.coordinates().element();
        let new_mouse = MousePos::from((coords.x, coords.y));
        mouse.set(new_mouse);

        if *is_panning.read() {
            update_pan_position(coords, last_mouse, pan);
            last_mouse.set(new_mouse);
        }
    }
}

pub fn handle_mouse_down(
    mut is_panning: Signal<bool>,
    mut last_mouse: Signal<MousePos>,
    mouse: Signal<MousePos>,
) -> impl FnMut(Event<MouseData>) {
    move |event: Event<MouseData>| {
        if should_start_panning(&event) {
            is_panning.set(true);
            last_mouse.set(*mouse.read());
        }
    }
}

pub fn handle_mouse_up(mut is_panning: Signal<bool>) -> impl FnMut(Event<MouseData>) {
    move |event: Event<MouseData>| {
        if should_stop_panning(&event) {
            is_panning.set(false);
        }
    }
}

pub fn handle_wheel(
    mut zoom: Signal<Zoom>,
    pan: Signal<Pan>,
    mouse: Signal<MousePos>,
) -> impl FnMut(Event<WheelData>) {
    move |event: Event<WheelData>| {
        event.prevent_default();
        let delta = event.data.delta().strip_units().y;

        zoom.with_mut(|z| {
            let old_zoom = z.0;
            apply_zoom_delta(z, delta);
            adjust_pan_for_zoom(pan, mouse, old_zoom, z.0);
        });
    }
}

// ========= Sous-fonctions avec conditions ========= //

fn should_start_panning(event: &Event<MouseData>) -> bool {
    event.data.held_buttons().contains(MouseButton::Auxiliary)
}

fn should_stop_panning(event: &Event<MouseData>) -> bool {
    !event.data.held_buttons().contains(MouseButton::Auxiliary)
}

fn update_pan_position(coords: ElementPoint, last_mouse: Signal<MousePos>, mut pan: Signal<Pan>) {
    let delta_x = coords.x - last_mouse.read().x;
    let delta_y = coords.y - last_mouse.read().y;

    pan.with_mut(|p| {
        p.x += delta_x;
        p.y += delta_y;
    });
}

fn apply_zoom_delta(zoom: &mut Zoom, delta: f64) {
    if delta < 0.0 {
        zoom.zoom_in();
    } else {
        zoom.zoom_out();
    }
}

fn adjust_pan_for_zoom(
    mut pan: Signal<Pan>,
    mouse: Signal<MousePos>,
    old_zoom: f64,
    new_zoom: f64,
) {
    let ratio = new_zoom / old_zoom;
    let pos = mouse.read();

    pan.with_mut(|p| {
        p.x = pos.x - (pos.x - p.x) * ratio;
        p.y = pos.y - (pos.y - p.y) * ratio;
    });
}

use super::events::*;
use super::frames::*;
use super::grid::*;
use super::models::*;
use super::utils::*;
use dioxus::prelude::*;

#[component]
pub fn Pointer(mouse: Signal<MousePos>, pan: Signal<Pan>, zoom: Signal<Zoom>) -> Element {
    let mouse_pos = *mouse.read();
    let (x_svg, y_svg) = screen_to_svg(
        mouse_pos.x,
        mouse_pos.y,
        (*pan.read()).into(),
        zoom.read().0,
    );

    rsx! {
        circle {
            cx: "{x_svg}",
            cy: "{y_svg}",
            r: "{5.0 / zoom.read().0}",
            fill: "#00ff88",
            stroke: "black",
            stroke_width: "{1.5 / zoom.read().0}"
        }
    }
}

#[component]
pub fn Canvas() -> Element {
    // Pixel World
    let mouse = use_signal(MousePos::default);
    let last_mouse = use_signal(MousePos::default);
    let canvas_size = use_signal(CanvasSize::default);

    // Svg World
    let pan = use_signal(Pan::default);
    let zoom = use_signal(|| Zoom(1.0));
    let is_panning = use_signal(|| false);

    let shapes = use_signal(|| {
        vec![
            Shape {
                id: 1,
                x: 150.0,
                y: 100.0,
                z: 0.0,
                width: 100.0,
                height: 100.0,
                depth: 0.0,
                color: "#FF6B6B".to_string(),
            },
            Shape {
                id: 2,
                x: 110.0,
                y: 140.0,
                z: 0.0,
                width: 150.0,
                height: 150.0,
                depth: 0.0,
                color: "#4ECDC4".to_string(),
            },
        ]
    });

    rsx! {
        svg {
            width: "100%",
            height: "80vh",
            style: "border: 0px solid #ccc; background: #f9fafb;",
            view_box: "0 0 {canvas_size.read().width} {canvas_size.read().height}",

            // Resize Event
            onresize: handle_resize(canvas_size),

            // Mouse event
            onmousemove: handle_mouse_move(mouse, last_mouse, pan, is_panning),
            onmousedown: handle_mouse_down(is_panning, last_mouse, mouse),
            onmouseup: handle_mouse_up(is_panning),
            onwheel: handle_wheel(zoom, pan, mouse),

            g {
                transform: "translate({pan.read().x}, {pan.read().y}) scale({zoom.read().0})",
                IsometricGrid { pan, zoom, canvas_size }
                ShapeList { shapes }
                Pointer { mouse, pan, zoom }
            }
        }

        div {
            style: "margin-top: 15px; padding: 10px; background: #f9fafb; font-family: monospace; font-size: 12px;",
            "Mouse: {mouse.read().x:.1} {mouse.read().y:.1}"
            br {}
            "Pan: {pan.read().x:.1} {pan.read().y:.1}"
            br {}
            "Zoom: {zoom.read().0:.2}"
            br {}
            "is_panning : {is_panning}"
            br {}
            "canvas_size: {canvas_size.read().width} X {canvas_size.read().height}"
        }
    }
}

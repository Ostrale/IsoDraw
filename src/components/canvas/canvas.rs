use super::events::*;
use super::frames::*;
use super::grid::*;
use super::models::*;
use super::utils::*;
use dioxus::prelude::*;

enum PointerState {
    Circle,
    Cube,
    None,
}

#[component]
pub fn Pointer(mouse: Signal<MousePos>, pan: Signal<Pan>, zoom: Signal<Zoom>) -> Element {
    let mouse_pos = *mouse.read();
    let (x_svg, y_svg) = screen_to_svg(
        mouse_pos.x,
        mouse_pos.y,
        (*pan.read()).into(),
        zoom.read().0,
    );

    let pointer_state_signal = use_context::<Signal<PointerState>>();
    let pointer_state = pointer_state_signal.read();

    match *pointer_state {
        PointerState::Cube => rsx! {
            ShapePolygon {
                shape: Shape::new(
                    0,
                    IsoCoord::new(x_svg, y_svg).into(),
                    Dimensions::new(1, 1, 1),
                    "#aa99aa".to_string()
                ).with_style(ShapeStyle::new(
                     "#aa99aa".to_string(),  // couleur de remplissage
                     "black".to_string(),
                     0.3,                    // sans bordure
                     0.4,                    // bordure transparente
                     0.6,                    // remplissage semi-transparent
                 ))
            },
        },
        PointerState::Circle => rsx! {
            circle {
                cx: "{x_svg}",
                cy: "{y_svg}",
                r: "{5.0 / zoom.read().0}",
                fill: "#00ff88",
                stroke: "black",
                stroke_width: "{1.5 / zoom.read().0}"
            },
        },
        PointerState::None => rsx! {},
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

    //
    let mut pointer_state = use_signal(|| PointerState::Circle);
    use_context_provider(|| pointer_state);

    let shapes: Signal<Vec<Shape>> = use_signal(|| {
        vec![
            Shape::new(
                3,
                GridCoord { x: 20, y: 1 },
                Dimensions::new(2, 2, 1),
                "#FF6B6B".to_string(),
            ),
            Shape::new(
                2,
                GridCoord { x: 20, y: 0 },
                Dimensions::new(1, 1, 3),
                "#FF6B6B".to_string(),
            ),
            Shape::new(
                1,
                GridCoord { x: 19, y: 0 },
                Dimensions::new(1, 1, 9),
                "#AE6B6B".to_string(),
            ),
        ]
    });

    rsx! {
        svg {
            width: "100%",
            height: "95vh",
            style: "border: 0px solid #ccc; background: #f9fafb;",
            //view_box: "0 0 {canvas_size.read().width} {canvas_size.read().height}",

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
            style: "padding: 10px; background: #f0f0f0; display: flex; gap: 10px; margin-bottom: 10px;",

            button {
                onclick: move |_| *pointer_state.write() = PointerState::Circle,
                style: "padding: 8px 16px; cursor: pointer;",
                "Circle"
            }

            button {
                onclick: move |_| *pointer_state.write() = PointerState::Cube,
                style: "padding: 8px 16px; cursor: pointer;",
                "Cube"
            }

            button {
                onclick: move |_| *pointer_state.write() = PointerState::None,
                style: "padding: 8px 16px; cursor: pointer;",
                "None"
            }
        }

        // div {
        //     style: "margin-top: 15px; padding: 10px; background: #f9fafb; font-family: monospace; font-size: 12px;",
        //     "Mouse: {mouse.read().x:.1} {mouse.read().y:.1}"
        //     br {}
        //     "Pan: {pan.read().x:.1} {pan.read().y:.1}"
        //     br {}
        //     "Zoom: {zoom.read().0:.2}"
        //     br {}
        //     "is_panning : {is_panning}"
        //     br {}
        //     "canvas_size: {canvas_size.read().width} X {canvas_size.read().height}"
        // }
    }
}

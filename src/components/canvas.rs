use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Shape {
    id: usize,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    height: f32,
    depth: f32,
    color: String,
}

#[component]
pub fn Canvas() -> Element {
    let mut shapes = use_signal(|| {
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
                x: 120.0,
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
            width: "800",
            height: "600",
            view_box: "0 0 800 600",
            style: "border: 1px solid #ccc; background: #f8f8f8;",

            // Rectangles avec un Shape
            {
                shapes.read().iter().map(|shape| rsx! {
                    rect {
                        x: "{shape.x}",
                        y: "{shape.y}",
                        width: "{shape.width}",
                        height: "{shape.height}",
                        fill: "{shape.color}",
                        stroke: "black"
                    }
                })
            }
        }
        button {
            onclick: move |_| shapes.with_mut(|list| {
                list[1].x += 10.0;
            }),
            "+10"
        }
        button {
            onclick: move |_| shapes.with_mut(|list| {
                list[1].x -= 10.0;
            }),
            "-10"
        }

    }
}

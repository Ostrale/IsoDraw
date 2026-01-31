use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Shape {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub color: String,
}

#[component]
pub fn ShapeList(shapes: Signal<Vec<Shape>>) -> Element {
    rsx! {
        for shape in shapes.read().iter() {
            rect {
                key: "{shape.id}",
                x: "{shape.x}",
                y: "{shape.y}",
                width: "{shape.width}",
                height: "{shape.height}",
                fill: "{shape.color}",
                stroke: "black"
            }
        }
    }
}

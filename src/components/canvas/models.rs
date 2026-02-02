use super::frames::{CartCoord, GridCoord, IsoCoord};
use super::grid::GRID_SIZE;
use dioxus::prelude::*;

// ============================================================================
// Dimensions
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn with_default(self, default_size: u32) -> Self {
        Self {
            width: if self.width > 0 {
                self.width
            } else {
                default_size
            },
            height: if self.height > 0 {
                self.height
            } else {
                default_size
            },
            depth: if self.depth > 0 {
                self.depth
            } else {
                default_size
            },
        }
    }
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth: 1,
        }
    }
}

// ============================================================================
// Style Properties
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeStyle {
    pub color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
    pub stroke_opacity: f64,
    pub fill_opacity: f64,
}

impl ShapeStyle {
    pub fn new(
        color: String,
        stroke_color: String,
        stroke_width: f64,
        stroke_opacity: f64,
        fill_opacity: f64,
    ) -> Self {
        Self {
            color,
            stroke_color,
            stroke_width,
            stroke_opacity: stroke_opacity.clamp(0.0, 1.0),
            fill_opacity: fill_opacity.clamp(0.0, 1.0),
        }
    }
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            color: "#808080".to_string(),
            stroke_color: "#000000".to_string(),
            stroke_width: 1.5,
            stroke_opacity: 1.0,
            fill_opacity: 1.0,
        }
    }
}

// ============================================================================
// Shape
// ============================================================================

#[derive(Clone, PartialEq)]
pub struct Shape {
    pub id: usize,
    pub position: GridCoord,
    pub dimensions: Dimensions,
    pub style: ShapeStyle,
}

impl Shape {
    pub fn new(id: usize, position: GridCoord, dimensions: Dimensions, color: String) -> Self {
        Self {
            id,
            position,
            dimensions,
            style: ShapeStyle {
                color,
                ..Default::default()
            },
        }
    }

    pub fn with_style(mut self, style: ShapeStyle) -> Self {
        self.style = style;
        self
    }

    pub fn iso_vertices(&self, default_size: u32) -> ShapeVertices {
        let dims = self.dimensions.with_default(default_size);
        let pos = self.position;

        let cart: CartCoord = pos.into();

        let x = cart.x;
        let y = cart.y;
        let z = cart.z;

        let dims_depth = dims.depth as f64 * GRID_SIZE;
        let dims_height = dims.height as f64 * GRID_SIZE;
        let dims_width = dims.width as f64 * GRID_SIZE;

        ShapeVertices {
            p1: CartCoord::new(x, y, z + dims_depth).into(),
            p2: CartCoord::new(x + dims_width, y, z + dims_depth).into(),
            p3: CartCoord::new(x + dims_width, y + dims_height, z + dims_depth).into(),
            p4: CartCoord::new(x, y + dims_height, z + dims_depth).into(),
            p5: CartCoord::new(x + dims_width, y, z).into(),
            p6: CartCoord::new(x + dims_width, y + dims_height, z).into(),
            p7: CartCoord::new(x, y + dims_height, z).into(),
        }
    }
}

// ============================================================================
// Shape Vertices
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct ShapeVertices {
    pub p1: IsoCoord,
    pub p2: IsoCoord,
    pub p3: IsoCoord,
    pub p4: IsoCoord,
    pub p5: IsoCoord,
    pub p6: IsoCoord,
    pub p7: IsoCoord,
}

impl ShapeVertices {
    pub fn as_array(&self) -> [IsoCoord; 7] {
        [
            self.p1, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7,
        ]
    }

    pub fn to_vec(&self) -> Vec<IsoCoord> {
        vec![
            self.p1, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7,
        ]
    }

    pub fn visible_faces(&self) -> ShapeFaces {
        ShapeFaces {
            top: vec![self.p1, self.p2, self.p3, self.p4],
            right: vec![self.p2, self.p5, self.p6, self.p3],
            left: vec![self.p4, self.p3, self.p6, self.p7],
        }
    }
}

// ============================================================================
// Faces
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceType {
    Top,
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct ShapeFaces {
    pub top: Vec<IsoCoord>,
    pub right: Vec<IsoCoord>,
    pub left: Vec<IsoCoord>,
}

// Fonction pour formater les points en string SVG
fn format_points(vertices: &[IsoCoord]) -> String {
    vertices
        .iter()
        .map(|v| format!("{},{}", v.x, v.y))
        .collect::<Vec<_>>()
        .join(" ")
}

// Fonction pour ajuster la luminosité d'une couleur hexadécimale
fn adjust_brightness(hex: &str, percent: i32) -> String {
    // Enlever le '#' si présent
    let hex_clean = hex.trim_start_matches('#');

    // Parser la valeur hexadécimale
    let num = u32::from_str_radix(hex_clean, 16).unwrap_or(0);

    // Extraire les composantes RGB
    let r = ((num >> 16) & 0xFF) as i32;
    let g = ((num >> 8) & 0xFF) as i32;
    let b = (num & 0xFF) as i32;

    // Appliquer l'ajustement et clamper entre 0 et 255
    let r = (r + percent).clamp(0, 255) as u32;
    let g = (g + percent).clamp(0, 255) as u32;
    let b = (b + percent).clamp(0, 255) as u32;

    // Recombiner et formater en hex
    let result = (r << 16) | (g << 8) | b;
    format!("#{:06x}", result)
}

#[component]
pub fn ShapeList(shapes: Signal<Vec<Shape>>) -> Element {
    rsx! {
        for shape in shapes.read().iter().rev() {
            ShapePolygon { shape: shape.clone() }
        }
    }
}

#[component]
pub fn ShapePolygon(shape: Shape) -> Element {
    let vertices = shape.iso_vertices(1);
    let faces = vertices.visible_faces();
    let selectable = if shape.id != 0 { "true" } else { "false" };

    rsx! {
        g {
            key: "{shape.id}",
            id: "{shape.id}",
            "inbreakable": "true",
            "selectable": "{selectable}",

            for (face_type, face_vertices, brightness) in [
                (FaceType::Top, &faces.top, 0),
                (FaceType::Right, &faces.right, -20),
                (FaceType::Left, &faces.left, -40),
            ] {
                polygon {
                    key: "{face_type:?}",
                    points: format_points(face_vertices),
                    fill: "{adjust_brightness(&shape.style.color, brightness)}",
                    fill_opacity: "{shape.style.fill_opacity}",
                    stroke: "{shape.style.stroke_color}",
                    stroke_width: "{shape.style.stroke_width}",
                    stroke_opacity: "{shape.style.stroke_opacity}",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}

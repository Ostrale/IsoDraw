// ========= MousePos ========= //

#[derive(Debug, Clone, Copy, Default)]
pub struct MousePos {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for MousePos {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

// ========= CanvasSize ========= //

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CanvasSize {
    pub width: f64,
    pub height: f64,
}

impl From<(f64, f64)> for CanvasSize {
    fn from((x, y): (f64, f64)) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
}

// ========= Pan ========= //

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pan {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Pan {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<Pan> for (f64, f64) {
    fn from(pan: Pan) -> Self {
        (pan.x, pan.y)
    }
}

// ========= Zoom ========= //

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Zoom(pub f64);

impl Zoom {
    pub fn zoom_in(&mut self) {
        self.0 = (self.0 * 1.1).clamp(0.1, 100.0);
    }
    pub fn zoom_out(&mut self) {
        self.0 = (self.0 * 0.9).clamp(0.1, 100.0);
    }
}

// ========= Fonctions ========= //

/// Conversion écran → SVG
pub fn screen_to_svg(
    screen_x: f64,   // coords.x (pixels écran)
    screen_y: f64,   // coords.y (pixels écran)
    pan: (f64, f64), // translation pixels écran
    zoom: f64,       // facteur d'échelle
) -> (f64, f64) {
    let (px, py) = pan;

    // Déplacer par pan (en pixels écran)
    let world_x = screen_x - px;
    let world_y = screen_y - py;

    // Zoom
    let svg_x = world_x / zoom;
    let svg_y = world_y / zoom;

    (svg_x, svg_y)
}

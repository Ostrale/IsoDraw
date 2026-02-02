use std::f64::consts::PI;
use super::grid::GRID_SIZE;

// Structure pour les coordonnées 2D (isométriques)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsoCoord {
    pub x: f64,
    pub y: f64,
}

// Structure pour les coordonnées 3D (cartésiennes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartCoord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Structure pour les coordonnées Grille
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
}

impl IsoCoord {
    pub fn new(x: f64, y: f64) -> Self {
        IsoCoord { x, y }
    }
}

impl CartCoord {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CartCoord { x, y, z }
    }
}

impl GridCoord {
    pub fn new(x: i32, y: i32) -> Self {
        GridCoord { x, y }
    }
}

// Conversion coordonnées cartésiennes -> isométriques
impl From<CartCoord> for IsoCoord {
    fn from(cart: CartCoord) -> Self {
        IsoCoord {
            x: (cart.x - cart.y) * (PI / 6.0).cos(),
            y: (cart.x + cart.y) * (PI / 6.0).sin() - cart.z,
        }
    }
}

// Conversion isométrique -> cartésien (approximation)
impl From<IsoCoord> for CartCoord {
    fn from(iso: IsoCoord) -> Self {
        let cos_val = (PI / 6.0).cos();
        let sin_val = (PI / 6.0).sin();

        let x = (iso.x / cos_val + iso.y / sin_val) / 2.0;
        let y = (iso.y / sin_val - iso.x / cos_val) / 2.0;

        CartCoord { x, y, z: 0.0 }
    }
}

// Conversion gille -> cartésien
impl From<GridCoord> for CartCoord {
    fn from(grid: GridCoord) -> Self {
        let x = grid.x as f64 * GRID_SIZE;
        let y = grid.y as f64 * GRID_SIZE;

        CartCoord { x, y, z: 0.0 }
    }
}

// Conversion grille -> isométrique (via CartCoord)
impl From<GridCoord> for IsoCoord {
    fn from(grid: GridCoord) -> Self {
        let cart: CartCoord = grid.into();
        cart.into()
    }
}

// Conversion cartésien -> grille (avec clamping)
impl From<CartCoord> for GridCoord {
    fn from(cart: CartCoord) -> Self {
        let x = (cart.x / GRID_SIZE).round() as i32;
        let y = (cart.y / GRID_SIZE).round() as i32;

        GridCoord { x, y }
    }
}

// Conversion isométriques -> grille (via CartCoord)
impl From<IsoCoord> for GridCoord {
    fn from(iso: IsoCoord) -> Self {
        let cart: CartCoord = iso.into();
        cart.into()
    }
}



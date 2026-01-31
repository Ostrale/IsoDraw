use std::f64::consts::PI;

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

// Conversion coordonnées cartésiennes -> isométriques
pub fn to_iso(x: f64, y: f64, z: f64) -> IsoCoord {
    IsoCoord {
        x: (x - y) * (PI / 6.0).cos(),
        y: (x + y) * (PI / 6.0).sin() - z,
    }
}

// Conversion isométrique -> cartésien (approximation)
pub fn from_iso(iso_x: f64, iso_y: f64) -> CartCoord {
    let cos_val = (PI / 6.0).cos();
    let sin_val = (PI / 6.0).sin();

    let x = (iso_x / cos_val + iso_y / sin_val) / 2.0;
    let y = (iso_y / sin_val - iso_x / cos_val) / 2.0;

    CartCoord { x, y, z: 0.0 }
}

use super::frames::*;
use super::utils::*;
use dioxus::prelude::*;

pub const GRID_SIZE: f64 = 30.0;

#[derive(Clone, PartialEq)]
struct GridLines {
    vertical: Vec<(f64, f64, f64, f64)>,
    horizontal: Vec<(f64, f64, f64, f64)>,
}

fn calculate_grid_step(zoom: f64) -> i32 {
    // Déterminer le pas d'affichage selon le zoom
    if zoom >= 0.8 {
        1 // Afficher toutes les lignes
    } else if zoom >= 0.5 {
        2 // Une ligne sur 2
    } else if zoom >= 0.25 {
        4 // Une ligne sur 4
    } else if zoom >= 0.125 {
        8 // Une ligne sur 8
    } else {
        16 // Une ligne sur 16
    }
}

fn calculate_visible_grid(
    pan_x: f64,
    pan_y: f64,
    zoom: f64,
    canvas_width: f64,
    canvas_height: f64,
) -> GridLines {
    // Calculer les 4 coins de l'écran en coordonnées SVG
    let margin = 200.0;
    let corners = [
        ((-pan_x - margin) / zoom, (-pan_y - margin) / zoom),
        (
            (canvas_width - pan_x + margin) / zoom,
            (-pan_y - margin) / zoom,
        ),
        (
            (-pan_x - margin) / zoom,
            (canvas_height - pan_y + margin) / zoom,
        ),
        (
            (canvas_width - pan_x + margin) / zoom,
            (canvas_height - pan_y + margin) / zoom,
        ),
    ];

    // Convertir tous les coins en coordonnées cartésiennes
    let cart_coords: Vec<CartCoord> = corners
        .iter()
        .map(|(x, y)| IsoCoord { x: *x, y: *y }.into())
        .collect();

    // Trouver les limites min/max sur tous les coins
    let cart_min_x = cart_coords
        .iter()
        .map(|c| c.x)
        .fold(f64::INFINITY, f64::min);
    let cart_max_x = cart_coords
        .iter()
        .map(|c| c.x)
        .fold(f64::NEG_INFINITY, f64::max);
    let cart_min_y = cart_coords
        .iter()
        .map(|c| c.y)
        .fold(f64::INFINITY, f64::min);
    let cart_max_y = cart_coords
        .iter()
        .map(|c| c.y)
        .fold(f64::NEG_INFINITY, f64::max);

    // Calculer le pas d'affichage selon le zoom
    let step = calculate_grid_step(zoom);

    // Trouver les indices de grille min/max
    let grid_min_x = (cart_min_x / GRID_SIZE).floor() as i32 - 5;
    let grid_max_x = (cart_max_x / GRID_SIZE).ceil() as i32 + 5;
    let grid_min_y = (cart_min_y / GRID_SIZE).floor() as i32 - 5;
    let grid_max_y = (cart_max_y / GRID_SIZE).ceil() as i32 + 5;

    // Ajuster les indices pour être alignés sur le pas
    let grid_min_x = (grid_min_x / step) * step;
    let grid_max_x = (grid_max_x / step) * step;
    let grid_min_y = (grid_min_y / step) * step;
    let grid_max_y = (grid_max_y / step) * step;

    // Extension pour que les lignes soient assez longues
    let extension = 1000.0;

    // Générer les lignes verticales (avec le pas)
    let vertical = (grid_min_x..=grid_max_x)
        .step_by(step as usize)
        .map(|i| {
            let x = i as f64 * GRID_SIZE;
            let y_min = cart_min_y - extension;
            let y_max = cart_max_y + extension;

            let start: IsoCoord = CartCoord { x: x, y: y_min, z: 0.0 }.into();
            let end: IsoCoord = CartCoord { x: x, y: y_max, z: 0.0 }.into();

            (start.x, start.y, end.x, end.y)
        })
        .collect();

    // Générer les lignes horizontales (avec le pas)
    let horizontal = (grid_min_y..=grid_max_y)
        .step_by(step as usize)
        .map(|i| {
            let y = i as f64 * GRID_SIZE;
            let x_min = cart_min_x - extension;
            let x_max = cart_max_x + extension;

            let start: IsoCoord = CartCoord { x: x_min, y, z: 0.0 }.into();
            let end: IsoCoord = CartCoord { x: x_max, y, z: 0.0 }.into();
            (start.x, start.y, end.x, end.y)
        })
        .collect();

    GridLines {
        vertical,
        horizontal,
    }
}

#[component]
pub fn IsometricGrid(
    pan: Signal<Pan>,
    zoom: Signal<Zoom>,
    canvas_size: Signal<CanvasSize>,
) -> Element {
    // Calculer les lignes de grille visibles
    let grid_lines = use_memo(move || {
        calculate_visible_grid(
            pan.read().x,
            pan.read().y,
            zoom.read().0,
            canvas_size.read().width,
            canvas_size.read().height,
        )
    });

    rsx! {
        g {
            stroke: "#b2b2b2",
            stroke_width: "{0.5 / zoom.read().0}",

            // Lignes verticales
            for (idx, (x1, y1, x2, y2)) in grid_lines().vertical.iter().enumerate() {
                line {
                    key: "v-{idx}",
                    x1: "{x1}",
                    y1: "{y1}",
                    x2: "{x2}",
                    y2: "{y2}",
                }
            }

            // Lignes horizontales
            for (idx, (x1, y1, x2, y2)) in grid_lines().horizontal.iter().enumerate() {
                line {
                    key: "h-{idx}",
                    x1: "{x1}",
                    y1: "{y1}",
                    x2: "{x2}",
                    y2: "{y2}",
                }
            }
        }
    }
}

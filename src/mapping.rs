use sdl2::{render::WindowCanvas, pixels::Color, rect::Point};

pub fn display(canvas: &mut WindowCanvas) {
    let (width, height) = canvas.output_size().unwrap();

    // Largeur d'une voie (on suppose une largeur de voie fixe, par exemple 40 pixels)
    let lane_width = 45;

    // Calcul des positions des lignes verticales et horizontales pour séparer les voies
    let v_lanes = [
        width as i32 / 2,
        width as i32 / 2 - 3 * lane_width, // Ligne gauche extérieure
        width as i32 / 2 - 2 * lane_width,
        width as i32 / 2 - lane_width,
        width as i32 / 2 + lane_width,    // Ligne droite intérieure
        width as i32 / 2 + 2 * lane_width,
        width as i32 / 2 + 3 * lane_width // Ligne droite extérieure
    ];

    let h_lanes = [
        height as i32 / 2,
        height as i32 / 2 - 3 * lane_width, // Ligne haute extérieure
        height as i32 / 2 - 2 * lane_width,
        height as i32 / 2 - lane_width,
        height as i32 / 2 + lane_width,    // Ligne basse intérieure
        height as i32 / 2 + 2 * lane_width,
        height as i32 / 2 + 3 * lane_width // Ligne basse extérieure
    ];

    // Définir la couleur de dessin pour les lignes de séparation
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Dessiner les lignes verticales séparant les voies
    for &v_lane in &v_lanes {
        canvas.draw_line(Point::new(v_lane, 0), Point::new(v_lane, height as i32)).unwrap();
    }

    // Dessiner les lignes horizontales séparant les voies
    for &h_lane in &h_lanes {
        canvas.draw_line(Point::new(0, h_lane), Point::new(width as i32, h_lane)).unwrap();
    }
}
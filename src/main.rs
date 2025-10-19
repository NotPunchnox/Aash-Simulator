extern crate kiss3d;

use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3};
use kiss3d::window::Window;

#[kiss3d::main]
async fn main() {
    // Fenêtre 3D
    let mut window = Window::new("Simulation vectorielle 3D - Kiss3d");

    // Caméra orbitale (autour du point (0, 0, 0))
    let eye = Point3::new(20.0, 20.0, 20.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);

    // Définition de deux vecteurs 3D & dimensions
    let coxa_length = 5.0;   // Longueur du coxa en cm
    let femur_length = 6.3;  // Longueur du femur en cm
    let tibia_length = 13.0; // Longueur du tibia en cm

    // Coxa: vecteur 1 | 5cm
    let start1 = Point3::new(0.0, 0.0, 0.0);
    let end1   = Point3::new(coxa_length, 0.0, 0.0);

    // Femur: vecteur 2 | 6.3cm
    let start2 = end1;
    let end2   = Point3::new(coxa_length + femur_length, 0.0, 0.0);

    // Tibia: vecteur 3 | 13cm
    let start3 = end2;
    let end3 = Point3::new(coxa_length + femur_length + tibia_length, 0.0, 1.0);

    // Boucle principale
    while window.render_with_camera(&mut camera).await {
        // Dessin du premier vecteur (vert)
        window.draw_line(&start1, &end1, &Point3::new(0.0, 1.0, 0.0));

        // Dessin du second vecteur (bleu)
        window.draw_line(&start2, &end2, &Point3::new(0.0, 0.0, 1.0));

        // Ligne reliant les deux vecteurs (rouge)
        window.draw_line(&end1, &end2, &Point3::new(1.0, 0.0, 0.0));

        // Dessin du troisième vecteur (jaune)
        window.draw_line(&start3, &end3, &Point3::new(1.0, 1.0, 0.0));

        // Axes de repère
        window.draw_line(&Point3::origin(), &Point3::new(1.0, 0.0, 0.0), &Point3::new(0.5, 0.5, 0.5));
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 1.0, 0.0), &Point3::new(0.5, 0.5, 0.5));
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 0.0, 1.0), &Point3::new(0.5, 0.5, 0.5));
    }
}
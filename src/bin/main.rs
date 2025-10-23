extern crate kiss3d;

use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{MatrixCross, Point3};
use kiss3d::window::Window;

use aash_simulator::robot::parts::{Leg, LegPosition};


fn create_leg() -> Leg {
    // Créer une patte
    Leg::new(1, 5.0, 6.3, 13.0)
}

#[kiss3d::main]
async fn main() {
    // Fenêtre 3D
    let mut window = Window::new("Simulation vectorielle 3D - Kiss3d");

    // Caméra orbitale (autour du point (0, 0, 0))
    let eye = Point3::new(20.0, 20.0, 20.0);
    let at  = Point3::origin();
    let mut camera = ArcBall::new(eye, at);

    // Coordonnées à atteindre
    let x: f32 = 15.0;
    let y: f32 =  1.0;
    let z: f32 = -5.0;

    // Instancier la patte
    let mut leg = create_leg();

    // Positionner la patte aux positions cibles
    LegPosition::set_position(&mut leg, x, y, z);
    let matrix_points = leg.get_matrix_points().unwrap();
    

    // Récupérer les coordonnées des articulations
    let c1 = Point3::new(0.0, 0.0, 0.0); // Coxa base
    let c2 = Point3::new(
        matrix_points.coxa.0 as f32,
        matrix_points.coxa.1 as f32,
        matrix_points.coxa.2 as f32,
    );
    let f1 = c2; // Femur base
    let f2 = Point3::new(
        matrix_points.femur.0 as f32,
        matrix_points.femur.1 as f32,
        matrix_points.femur.2 as f32,
    );
    let t1 = f2; // Tibia base
    let t2 = Point3::new(
        matrix_points.tibia.0 as f32,
        matrix_points.tibia.1 as f32,
        matrix_points.tibia.2 as f32,
    );

    // Boucle principale
    while window.render_with_camera(&mut camera).await {
        // Dessin du premier vecteur (vert)
        window.draw_line(&c1, &c2, &Point3::new(0.0, 1.0, 0.0));

        // Dessin du second vecteur (bleu)
        window.draw_line(&f1, &f2, &Point3::new(0.0, 0.0, 1.0));

        // Ligne reliant les deux vecteurs (rouge)
        window.draw_line(&c2, &f2, &Point3::new(1.0, 0.0, 0.0));

        // Dessin du troisième vecteur (jaune)
        window.draw_line(&t1, &t2, &Point3::new(1.0, 1.0, 0.0));

        // Axes de repère
        window.draw_line(&Point3::origin(), &Point3::new(1.0, 0.0, 0.0), &Point3::new(0.5, 0.5, 0.5));
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 1.0, 0.0), &Point3::new(0.5, 0.5, 0.5));
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 0.0, 1.0), &Point3::new(0.5, 0.5, 0.5));
    }
}

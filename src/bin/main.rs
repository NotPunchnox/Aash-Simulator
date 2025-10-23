extern crate kiss3d;

use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3};
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
    // Convention moteur de jeu: X=avant/arrière, Y=gauche/droite, Z=haut/bas
    let x: f32 = 15.0;  // Avant/Arrière
    let y: f32 =  1.0;  // Gauche/Droite
    let z: f32 = -5.0;  // Haut/Bas (vertical)

    // Instancier la patte
    let mut leg = create_leg();

    // Positionner la patte aux positions cibles
    LegPosition::set_position(&mut leg, x, y, z);
    let matrix_points = leg.get_matrix_points().unwrap();
    
    // Afficher les coordonnées des articulations (MatrixPoint.print_summary())
    matrix_points.print_summary();

    // Boucle principale
    while window.render_with_camera(&mut camera).await {

        let (c1, c2, f1, f2, t1, t2, e1, e2) = LegPosition::get_leg_joints(&leg);
        
        // Dessiner la patte
        window.draw_line(&c1, &c2, &Point3::new(1.0, 0.0, 0.0)); // Coxa
        window.draw_line(&f1, &f2, &Point3::new(0.0, 1.0, 0.0)); // Fémur
        window.draw_line(&t1, &t2, &Point3::new(0.0, 0.0, 1.0)); // Tibia
        window.draw_line(&e1, &e2, &Point3::new(1.0, 1.0, 0.0)); // End

        // Axes de repère (convention moteur de jeu: X=rouge, Y=vert, Z=bleu vertical)
        window.draw_line(&Point3::origin(), &Point3::new(50.0, 0.0, 0.0), &Point3::new(1.0, 0.0, 0.0)); // X (rouge)
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 50.0, 0.0), &Point3::new(0.0, 1.0, 0.0)); // Y (vert)
        window.draw_line(&Point3::origin(), &Point3::new(0.0, 0.0, 50.0), &Point3::new(0.0, 0.0, 1.0)); // Z (bleu, vertical)
    }
}

extern crate kiss3d;

use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3};
use kiss3d::window::Window;

use aash_simulator::robot::parts::{Leg, LegPosition};


#[kiss3d::main]
async fn main() {
    // Fenêtre 3D
    let mut window = Window::new("Simulation vectorielle 3D - Kiss3d");

    // Caméra orbitale (autour du point (0, 0, 0))
    let eye = Point3::new(20.0, 20.0, 20.0);
    let at  = Point3::origin();
    let mut camera = ArcBall::new(eye, at);

    // Instanciation d'une patte;
    let leg  = Leg::new(1, 5.0, 6.3, 13.0);

    // Coxa: vecteur 1 | 5cm
    let c1   = Point3::new(0.0, 0.0, 0.0);
    let c2   = Point3::new(leg.get_coxa_length(), 1.0, 0.0);

    // Femur: vecteur 2 | 6.3cm
    let f1   = c2;
    let f2   = Point3::new(leg.get_coxa_length() + leg.get_femur_length(), 0.0, 0.0);

    // Tibia: vecteur 3 | 13cm
    let t1 = f2;
    let t2 = Point3::new(leg.get_coxa_length() + leg.get_femur_length() + leg.get_tibia_length(), 0.0, 0.0);

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

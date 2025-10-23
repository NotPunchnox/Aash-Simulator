use aash_simulator::robot::parts::{Leg, LegPosition};

fn main() {
    println!("===== Instance de test pour la conception d'une patte et la cinématique inverse. =====\n");

    // Créer une patte avec les dimensions du programme trigo-calc
    let mut leg = Leg::new(1, 5.0, 6.3, 13.0);
    
    // Définir une position cible (comme dans main.rs: x=20, y=-5, z=-5)
    LegPosition::set_position(&mut leg, 20.0, -5.0, -5.0);

    let position = leg.get_position();
    println!("Position cible: X: {:.2} | Y: {:.2} | Z: {:.2}", 
             position.x, position.y, position.z);
    
    // Calculer les angles
    match leg.get_angles() {
        Some(angles) => {
            println!("\n=== Angles calculés (en degrés) ===");
            println!("Theta 1 (Coxa)  : {:.2}°", angles.theta1);
            println!("Theta 4 (Fémur) : {:.2}°", angles.theta4);
            println!("Theta 5 (Tibia) : {:.2}°", angles.theta5);
        }
        None => {
            println!("Erreur: position cible non atteignable !");
            return;
        }
    }
    
    // Calculer toutes les coordonnées des articulations
    match leg.get_matrix_points() {
        Some(points) => {
            println!("\n=== Coordonnées des articulations (MatrixPoint) ===");
            println!("Coxa   : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                     points.coxa.0, points.coxa.1, points.coxa.2);
            println!("Fémur  : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                     points.femur.0, points.femur.1, points.femur.2);
            println!("Tibia  : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                     points.tibia.0, points.tibia.1, points.tibia.2);
            println!("Fin    : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                     points.end.0, points.end.1, points.end.2);
            
            println!("\n=== Longueurs des segments calculées ===");
            println!("Coxa   : {:.2} cm", points.coxa_length());
            println!("Fémur  : {:.2} cm", points.femur_length());
            println!("Tibia  : {:.2} cm", points.tibia_length());
            println!("Total  : {:.2} cm", points.total_length());
            
            println!("\n=== Vérification ===");
            println!("Longueurs définies: Coxa: {:.2} | Fémur: {:.2} | Tibia: {:.2}", 
                     leg.coxa_length, leg.femur_length, leg.tibia_length);
        }
        None => {
            println!("Erreur: impossible de calculer les coordonnées des articulations !");
        }
    }
}
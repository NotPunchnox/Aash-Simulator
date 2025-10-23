use aash_simulator::robot::parts::{Leg, LegPosition};

fn test_position(leg: &mut Leg, x: f32, y: f32, z: f32) {
    println!("\n{}", "=".repeat(60));
    println!("Test position: X: {:.2} | Y: {:.2} | Z: {:.2}", x, y, z);
    println!("{}", "=".repeat(60));
    
    LegPosition::set_position(leg, x, y, z);
    
    match leg.get_angles() {
        Some(angles) => {
            println!("✓ Position atteignable !");
            println!("\nAngles calculés:");
            println!("  Theta 1 (Coxa)  : {:.2}°", angles.theta1);
            println!("  Theta 4 (Fémur) : {:.2}°", angles.theta4);
            println!("  Theta 5 (Tibia) : {:.2}°", angles.theta5);
            
            if let Some(points) = leg.get_matrix_points() {
                println!("\nCoordonnées des articulations:");
                println!("  Coxa   : ({:.2}, {:.2}, {:.2})", 
                         points.coxa.0, points.coxa.1, points.coxa.2);
                println!("  Fémur  : ({:.2}, {:.2}, {:.2})", 
                         points.femur.0, points.femur.1, points.femur.2);
                println!("  Tibia  : ({:.2}, {:.2}, {:.2})", 
                         points.tibia.0, points.tibia.1, points.tibia.2);
                println!("  Fin    : ({:.2}, {:.2}, {:.2})", 
                         points.end.0, points.end.1, points.end.2);
                
                println!("\nVérification des longueurs:");
                println!("  Coxa   : {:.2} cm (définie: {:.2} cm)", 
                         points.coxa_length(), leg.coxa_length);
                println!("  Fémur  : {:.2} cm (définie: {:.2} cm)", 
                         points.femur_length(), leg.femur_length);
                println!("  Tibia  : {:.2} cm (définie: {:.2} cm)", 
                         points.tibia_length(), leg.tibia_length);
                println!("  Total  : {:.2} cm", points.total_length());
            }
        }
        None => {
            println!("✗ Position NON atteignable !");
            println!("  Raison: La cible est hors de portée de la patte.");
            
            let max_reach = leg.coxa_length + leg.femur_length + leg.tibia_length;
            let distance = (x * x + y * y + z * z).sqrt();
            println!("  Portée maximale : {:.2} cm", max_reach);
            println!("  Distance cible  : {:.2} cm", distance);
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║ Test de cinématique inverse avec la structure MatrixPoint  ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    
    println!("\nConvention des axes:");
    println!("  X = devant/derrière");
    println!("  Y = haut/bas (hauteur)");
    println!("  Z = gauche/droite");
    
    // Créer une patte ( dimensions en cm )
    let mut leg = Leg::new(1, 5.0, 6.3, 13.0);
    
    println!("\nDimensions de la patte:");
    println!("  Coxa  : {:.2} cm", leg.coxa_length);
    println!("  Fémur : {:.2} cm", leg.femur_length);
    println!("  Tibia : {:.2} cm", leg.tibia_length);
    println!("  Total : {:.2} cm", leg.coxa_length + leg.femur_length + leg.tibia_length);
    
    // Test 1: Position du programme
    test_position(&mut leg, 20.0, -5.0, -5.0);
    
    // Test 2: Position simple sur l'axe X
    test_position(&mut leg, 15.0, 0.0, 0.0);
    
    // Test 3: Position avec élévation positive
    test_position(&mut leg, 10.0, 5.0, 8.0);
    
    // Test 4: Position à la limite de portée
    test_position(&mut leg, 24.0, 0.0, 0.0);
    
    // Test 5: Position hors de portée (devrait échouer)
    test_position(&mut leg, 30.0, 10.0, 10.0);
    
    println!("\n{}", "=".repeat(60));
    println!("Tests terminés !");
}

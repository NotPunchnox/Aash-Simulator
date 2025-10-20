use aash_simulator::robot::parts::{Leg, LegPosition};

fn main() {
    println!("===== Instance de test pour la conception d'une patte et la cinématique inverse. =====\n");

    let mut leg = Leg::new(1, 10.0, 20.0, 30.0);
    LegPosition::set_position(&mut leg, 10.0, 10.0, 15.0);


    let position = leg.get_position();
    let angles = leg.get_angles();
    
    println!("Position: {:?}", position);
    println!("Angles: {:?}", angles);
}
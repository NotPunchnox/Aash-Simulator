use Aash_simulator::robot::parts::{Leg, LegPosition};

fn main() {
    println!("Instance de test pour la conception d'une patte et la cinématique inverse.");

    let mut leg = Leg::new(1, 10.0, 20.0, 30.0);
    LegPosition::set_position(&mut leg, 5.0, 10.0, 15.0);


    let angles = leg.get_angles(5.0, 10.0, 15.0);
    let position = leg.get_position();
    
    println!("Position: {:?}", position);
    println!("Angles: {:?}", angles);
}
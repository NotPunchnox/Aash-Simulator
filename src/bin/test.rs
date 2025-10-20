use Aash_simulator::robot::parts::{Leg, LegPosition};

fn main() {
    println!("Instance de test pour la conception d'une patte et la cinématique inverse.");

    let leg = Leg::new(1, 10.0, 20.0, 30.0);
    let angles = leg.get_angles(5.0, 10.0, 15.0);
    println!("Angles: {:?}", angles);
}
use crate::robot::utils::*;
use super::leg::Leg;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Angles {
    pub coxa_angle: f32,
    pub femur_angle: f32,
    pub tibia_angle: f32,
}

pub trait LegPosition {
    fn set_position(&mut self, x: f32, y: f32, z: f32);
    fn get_position(&self) -> Position;
    fn get_angles(&self) -> Angles;
}

impl LegPosition for Leg {

    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn get_position(&self) -> Position {
        Position { x: self.x, y: self.y, z: self.z }
    }

    fn get_angles(&self) -> Angles {
        let position = self.get_position();
        let tpatte = racine(position.x, position.y);
        let hypotenuse = racine(position.z, tpatte - self.coxa_length);

        let angle_a = radians_to_degrees(atan(tpatte - self.coxa_length / position.z));
        let angle_b = al_kashi(self.femur_length, hypotenuse, self.tibia_length);
        let angles_femur = angle_a + angle_b;

        let angles_tibia = al_kashi(self.tibia_length, self.femur_length, hypotenuse);
        let angles_coxa = radians_to_degrees(atan(position.y / position.x)) + 90.0;
        
        Angles {
            coxa_angle: angles_coxa,
            femur_angle: angles_femur,
            tibia_angle: angles_tibia,
        }
    }

}

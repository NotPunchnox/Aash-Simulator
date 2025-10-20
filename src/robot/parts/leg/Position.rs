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
    fn get_angles(&self, x: f32, y: f32, z: f32) -> Angles;
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

    fn get_angles(&self, x: f32, y: f32, z: f32) -> Angles {
        let TPatte = Racine(x, y);
        let hypotenuse = Racine(z, TPatte - self.coxa_length);

        let angle_a = radiansToDegrees(atan(TPatte - self.coxa_length / z));
        let angle_b = AlKashi(self.femur_length, hypotenuse, self.tibia_length);
        let angles_femur = angle_a + angle_b;

        let angles_tibia = AlKashi(self.tibia_length, self.femur_length, hypotenuse);
        let angles_coxa = radiansToDegrees(atan(y / x)) + 90.0;
        
        Angles {
            coxa_angle: angles_coxa,
            femur_angle: angles_femur,
            tibia_angle: angles_tibia,
        }
    }

}

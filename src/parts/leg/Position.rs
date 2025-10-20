#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub coxa_angle: f32,
    pub femur_angle: f32,
    pub tibia_angle: f32,
}

impl Position for Leg {

    // Getters
    fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    // Setters
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;

    }

    // Inverse Kinematics function
    fn inverse_kinematics(&self, x: f32, y: f32, z: f32) {
        
    }

}

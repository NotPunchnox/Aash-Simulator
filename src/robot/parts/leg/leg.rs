#[derive(Debug)]
pub struct Leg {
    pub id: u32,
    pub coxa_length: f32,
    pub femur_length: f32,
    pub tibia_length: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Leg {

    // Constructeur
    pub fn new(id: u32, coxa_length: f32, femur_length: f32, tibia_length: f32) -> Self {
        Self {
            id,
            coxa_length,
            femur_length,
            tibia_length,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // Getters & Setters
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_coxa_length(&self) -> f32 {
        self.coxa_length
    }
    pub fn get_femur_length(&self) -> f32 {
        self.femur_length
    }
    pub fn get_tibia_length(&self) -> f32 {
        self.tibia_length
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn set_coxa_length(&mut self, length: f32) {
        self.coxa_length = length;
    }
    pub fn set_femur_length(&mut self, length: f32) {
        self.femur_length = length;
    }
    pub fn set_tibia_length(&mut self, length: f32) {
        self.tibia_length = length;
    }

}
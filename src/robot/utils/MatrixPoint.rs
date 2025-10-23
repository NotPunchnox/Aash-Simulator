// src/robot/utils/MatrixPoint.rs


// Structure pour stocker les coordonnées des articulations d'une patte de robot
#[derive(Clone, Debug)]
pub struct MatrixPoint {
    pub coxa: (f64, f64, f64),
    pub femur: (f64, f64, f64),
    pub tibia: (f64, f64, f64),
    pub end: (f64, f64, f64),
}

impl MatrixPoint {
    /// MatrixPoint avec toutes les coordonnées
    pub fn new(
        coxa: (f64, f64, f64),
        femur: (f64, f64, f64),
        tibia: (f64, f64, f64),
        end: (f64, f64, f64),
    ) -> Self {
        Self {
            coxa,
            femur,
            tibia,
            end,
        }
    }

    /// Calcule la longueur du segment Coxa (de coxa à femur)
    pub fn coxa_length(&self) -> f64 {
        let dx = self.femur.0 - self.coxa.0;
        let dy = self.femur.1 - self.coxa.1;
        let dz = self.femur.2 - self.coxa.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur du segment Fémur (de femur à tibia)
    pub fn femur_length(&self) -> f64 {
        let dx = self.tibia.0 - self.femur.0;
        let dy = self.tibia.1 - self.femur.1;
        let dz = self.tibia.2 - self.femur.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur du segment Tibia (de tibia à end)
    pub fn tibia_length(&self) -> f64 {
        let dx = self.end.0 - self.tibia.0;
        let dy = self.end.1 - self.tibia.1;
        let dz = self.end.2 - self.tibia.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur totale de la patte
    pub fn total_length(&self) -> f64 {
        self.coxa_length() + self.femur_length() + self.tibia_length()
    }

    /// Affiche toutes les coordonnées des articulations
    pub fn print_coordinates(&self) {
        println!("\n=== Coordonnées des articulations (MatrixPoint) ===");
        println!("Coxa   : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                 self.coxa.0, self.coxa.1, self.coxa.2);
        println!("Fémur  : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                 self.femur.0, self.femur.1, self.femur.2);
        println!("Tibia  : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                 self.tibia.0, self.tibia.1, self.tibia.2);
        println!("Fin    : X: {:.2} | Y: {:.2} | Z: {:.2}", 
                 self.end.0, self.end.1, self.end.2);
    }

    /// Affiche un résumé des longueurs des segments
    pub fn print_lengths(&self) {
        println!("\n=== Longueurs des segments calculées ===");
        println!("Coxa   : {:.2} cm", self.coxa_length());
        println!("Fémur  : {:.2} cm", self.femur_length());
        println!("Tibia  : {:.2} cm", self.tibia_length());
        println!("Total  : {:.2} cm", self.total_length());
    }

    /// Affiche un résumé complet (coordonnées + longueurs)
    pub fn print_summary(&self) {
        self.print_coordinates();
        self.print_lengths();
    }
}
use super::leg::Leg;
use kiss3d::nalgebra::Point3;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Angles {
    pub theta1: f32,  // Coxa - angle azimuthal (rotation autour de Z)
    pub theta4: f32,  // Fémur - angle d'élévation
    pub theta5: f32,  // Tibia - angle entre fémur et tibia
}

/// Structure pour stocker toutes les coordonnées 3D des articulations
#[derive(Debug, Clone)]
pub struct MatrixPoint {
    pub coxa: (f32, f32, f32),
    pub femur: (f32, f32, f32),
    pub tibia: (f32, f32, f32),
    pub end: (f32, f32, f32),
}

impl MatrixPoint {
    /// Crée un nouveau MatrixPoint avec toutes les coordonnées
    pub fn new(
        coxa: (f32, f32, f32),
        femur: (f32, f32, f32),
        tibia: (f32, f32, f32),
        end: (f32, f32, f32),
    ) -> Self {
        Self {
            coxa,
            femur,
            tibia,
            end,
        }
    }

    /// Calcule la longueur du segment Coxa (de coxa à femur)
    pub fn coxa_length(&self) -> f32 {
        let dx = self.femur.0 - self.coxa.0;
        let dy = self.femur.1 - self.coxa.1;
        let dz = self.femur.2 - self.coxa.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur du segment Fémur (de femur à tibia)
    pub fn femur_length(&self) -> f32 {
        let dx = self.tibia.0 - self.femur.0;
        let dy = self.tibia.1 - self.femur.1;
        let dz = self.tibia.2 - self.femur.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur du segment Tibia (de tibia à end)
    pub fn tibia_length(&self) -> f32 {
        let dx = self.end.0 - self.tibia.0;
        let dy = self.end.1 - self.tibia.1;
        let dz = self.end.2 - self.tibia.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calcule la longueur totale de la patte
    pub fn total_length(&self) -> f32 {
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

pub trait LegPosition {
    fn get_leg_joints(&self) -> (Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>, Point3<f32>);
    fn set_position(&mut self, x: f32, y: f32, z: f32);
    fn get_position(&self) -> Position;
    fn get_angles(&self) -> Option<Angles>;
    fn get_matrix_points(&self) -> Option<MatrixPoint>;
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

    /// Calcule les angles des articulations en utilisant la cinématique inverse
    /// Basé sur le calcul du programme trigo-calc
    fn get_angles(&self) -> Option<Angles> {
        let position = self.get_position();
        
        // Distance horizontale projetée
        let tpatte = (position.x * position.x + position.y * position.y).sqrt();
        
        // Theta 1: angle azimuthal du coxa (rotation autour de Z)
        let theta1 = position.y.atan2(position.x);
        
        // Distance horizontale réelle après rotation du coxa
        let horizontal_dist = tpatte - self.coxa_length;
        let h = (position.z * position.z + horizontal_dist * horizontal_dist).sqrt();
        
        // Application de la loi des cosinus pour trouver l'angle entre fémur et tibia
        let cos_theta5 = (self.femur_length * self.femur_length 
                         + self.tibia_length * self.tibia_length 
                         - h * h) / (2.0 * self.femur_length * self.tibia_length);
        
        // Vérifier que le calcul est valide (position atteignable)
        if cos_theta5 > 1.0 || cos_theta5 < -1.0 {
            // Position non atteignable
            return None;
        }
        
        // Angle entre fémur et tibia
        let theta5 = cos_theta5.acos();
        
        // Calcul de l'angle du fémur ((a² + b² - c²) / (2ab))
        let cos_angle_au_coxa = (self.femur_length * self.femur_length + h * h  - self.tibia_length * self.tibia_length) / (2.0 * self.femur_length * h);
        let angle_au_coxa = cos_angle_au_coxa.acos();
        
        // Angle d'élévation du vecteur cible par rapport au coxa
        let target_horiz_dist = tpatte - self.coxa_length;
        let elev_angle = position.z.atan2(target_horiz_dist);
        
        let theta4 = elev_angle + angle_au_coxa;
        
        Some(Angles {
            theta1: theta1.to_degrees(),
            theta4: theta4.to_degrees(),
            theta5: theta5.to_degrees(),
        })
    }

    /// Calcule toutes les coordonnées 3D des articulations
    fn get_matrix_points(&self) -> Option<MatrixPoint> {
        let position = self.get_position();
        let angles = self.get_angles()?;
        
        // Convertir les angles en radians pour les calculs
        let theta1 = angles.theta1.to_radians();
        let theta4 = angles.theta4.to_radians();
        
        // Origine: base du coxa
        let coxa = (0.0, 0.0, 0.0);
        
        // Extrémité du coxa après rotation theta1
        let femur = (
            self.coxa_length * theta1.cos(),
            self.coxa_length * theta1.sin(),
            0.0
        );
        
        // Extrémité du fémur: extension dans le plan vertical orienté par theta1
        let tibia = (
            femur.0 + self.femur_length * theta1.cos() * theta4.cos(),
            femur.1 + self.femur_length * theta1.sin() * theta4.cos(),
            femur.2 + self.femur_length * theta4.sin()
        );
        
        // Position finale: extension du tibia vers la cible
        // Normalisation du vecteur tibia->cible pour respecter la longueur TIBIA
        let to_target_x = position.x - tibia.0;
        let to_target_y = position.y - tibia.1;
        let to_target_z = position.z - tibia.2;
        let dist_to_target = (to_target_x * to_target_x 
                            + to_target_y * to_target_y 
                            + to_target_z * to_target_z).sqrt();
        
        // Vecteur tibia normalisé à la longueur exacte
        let end = (
            tibia.0 + self.tibia_length * to_target_x / dist_to_target,
            tibia.1 + self.tibia_length * to_target_y / dist_to_target,
            tibia.2 + self.tibia_length * to_target_z / dist_to_target
        );
        
        Some(MatrixPoint::new(coxa, femur, tibia, end))
    }

    fn get_leg_joints(&self) -> (
        Point3<f32>, // c1
        Point3<f32>, // c2
        Point3<f32>, // f1
        Point3<f32>, // f2
        Point3<f32>, // t1
        Point3<f32>, // t2
        Point3<f32>, // e1
        Point3<f32>, // e2
    ) {
        // Récupérer les coordonnées des articulations

        let matrix_points = self.get_matrix_points().unwrap();

        let c1 = Point3::new(0.0, 0.0, 0.0); // Coxa base
        let c2 = Point3::new(
            matrix_points.coxa.0 as f32,
            matrix_points.coxa.1 as f32,
            matrix_points.coxa.2 as f32,
        );
        let f1 = c2; // Femur base
        let f2 = Point3::new(
            matrix_points.femur.0 as f32,
            matrix_points.femur.1 as f32,
            matrix_points.femur.2 as f32,
        );
        let t1 = f2; // Tibia base
        let t2 = Point3::new(
            matrix_points.tibia.0 as f32,
            matrix_points.tibia.1 as f32,
            matrix_points.tibia.2 as f32,
        );
        let e1 = t2; // End base
        let e2 = Point3::new(
            matrix_points.end.0 as f32,
            matrix_points.end.1 as f32,
            matrix_points.end.2 as f32,
        );

        return (c1, c2, f1, f2, t1, t2, e1, e2);
    }

}

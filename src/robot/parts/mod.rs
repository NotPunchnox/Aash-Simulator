// src/robot/parts/mod.rs

// Déclarer le module
pub mod leg;

// Réexporter les structures pour un accès plus facile
pub use leg::Leg;
pub use leg::Position;
pub use leg::Angles;
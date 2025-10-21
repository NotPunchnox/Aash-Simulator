// utils/math.rs
pub fn radians_to_degrees(rad: f32) -> f32 {
    return rad * (180.0 / std::f32::consts::PI);
}

pub fn degrees_to_radians(deg: f32) -> f32 {
    return deg * (std::f32::consts::PI / 180.0);
}

pub fn al_kashi(a: f32, b: f32, c: f32) -> f32 {
    let numerator = b.powi(2) + c.powi(2) - a.powi(2);
    let denominator = 2.0 * b * c;
    let angle_rad = (numerator / denominator).acos();

    return radians_to_degrees(angle_rad);
}

pub fn racine(x: f32, b: f32) -> f32 {
    return (x.powi(2) + b.powi(2)).sqrt();
}

pub fn atan(x: f32) -> f32 {
    return x.atan();
}

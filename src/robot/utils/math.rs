// utils/math.rs
pub fn radiansToDegrees(rad: f32) -> f32 {
    return rad * (180.0 / std::f32::consts::PI);
}

pub fn AlKashi(a: f32, b: f32, c: f32) -> f32 {
    let numerator = b.powi(2) + c.powi(2) - a.powi(2);
    let denominator = 2.0 * b * c;
    let angle_rad = (numerator / denominator).acos();

    return radiansToDegrees(angle_rad);
}

pub fn Racine(x: f32, b: f32) -> f32 {
    return (x.powi(2) + b.powi(2)).sqrt();
}

pub fn atan(x: f32) -> f32 {
    return x.atan();
}
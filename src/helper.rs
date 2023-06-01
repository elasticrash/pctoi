pub fn gtr(r: f32) -> f32 {
    (r / 200f32) * std::f32::consts::PI
}

pub fn distance(xa: f32, ya: f32, za: f32, xb: f32, yb: f32, zb: f32) -> f32 {
    ((xb - xa).powi(2) + (yb - ya).powi(2) + (zb - za).powi(2)).sqrt()
}

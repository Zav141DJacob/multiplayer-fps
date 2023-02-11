pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn fix_angle(deg: f32) -> f32 {
    let mut a = deg;
    if a > 359.0 {
        a -= 360.0;
    }
    if a < 0.0 {
        a += 360.0;
    }
    a
}
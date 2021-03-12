use crate::geometry::{Point2f, Vector3f};
use std::f32::consts::PI;

pub fn uniform_sample_sphere(u: Point2f) -> Vector3f {
    let z = 1.0 - 2.0 * u.x;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;

    Vector3f::new(r * phi.cos(), r * phi.sin(), z)
}

pub fn uniform_cone_pdf(cos_theta_max: f32) -> f32 {
    1.0 / (2.0 * PI * (1.0 - cos_theta_max))
}

use crate::geometry::{Point2f, Vector2f, Vector3f};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

pub fn uniform_sample_sphere(u: Point2f) -> Vector3f {
    let z = 1.0 - 2.0 * u.x;
    let r = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * PI * u.y;

    Vector3f::new(r * phi.cos(), r * phi.sin(), z)
}

pub fn uniform_cone_pdf(cos_theta_max: f32) -> f32 {
    1.0 / (2.0 * PI * (1.0 - cos_theta_max))
}

pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    // Map uniform random numbers to `[-1, 1]^2`
    let u_offset = 2.0 * u - Vector2f::new(1.0, 1.0);

    // Handle degeneracy at the origin
    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        return Point2f::new(0.0, 0.0);
    }

    // Apply concentric mapping to point
    let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
        (u_offset.x, FRAC_PI_4 * (u_offset.y / u_offset.x))
    } else {
        (
            u_offset.y,
            FRAC_PI_2 - FRAC_PI_4 * (u_offset.x / u_offset.y),
        )
    };
    r * Point2f::new(theta.cos(), theta.sin())
}

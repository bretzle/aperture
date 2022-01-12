use crate::{
    geometry::{differential_geometry::DifferentialGeometry, Boundable, Geometry, Sampleable},
    math::{self, Normal, Ray, Vector},
};
use std::f32::consts::PI;

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry> {
        // Compute quadratic coefficients for sphere intersection equation
        let a = ray.d.length_sqr();
        let b = 2.0 * math::dot(&ray.d, &ray.o);
        let c = math::dot(&ray.o, &ray.o) - self.radius * self.radius;
        // Try to solve the quadratic equation to find the candidate hit t values
        // if there are no solutions then we definitely don't hit the sphere
        let t = match math::solve_quadratic(a, b, c) {
            Some(x) => x,
            None => return None,
        };
        // Test that we're within the range of t values the ray is querying
        if t.0 > ray.max_t || t.1 < ray.min_t {
            return None;
        }
        // Find the first t value within the ray's range we hit
        let mut t_hit = t.0;
        if t_hit < ray.min_t {
            t_hit = t.1;
            if t_hit > ray.max_t {
                return None;
            }
        }
        // We have a valid hit if we get here, so fill out the ray max_t and
        // differential geometry info to send back
        ray.max_t = t_hit;
        let p = ray.at(t_hit);
        let n = Normal::new(p.x, p.y, p.z);
        let theta = f32::acos(math::clamp(p.z / self.radius, -1.0, 1.0));

        // Compute derivatives for point vs. parameterization
        let inv_z = 1.0 / f32::sqrt(p.x * p.x + p.y * p.y);
        let cos_phi = p.x * inv_z;
        let sin_phi = p.y * inv_z;
        // TODO: It doesn't make sense that dp_du x dp_dv and n point it such different
        // directions, they should at least point in a similar direction
        // Doing dp_dv x dp_du gives the same as normal, kind of as we'd expect since they're
        // facing opposite directions, but it doesn't explain why this would be wrong
        let u = match f32::atan2(p.x, p.y) / (2.0 * PI) {
            x if x < 0.0 => x + 1.0,
            x => x,
        };
        let v = theta / PI;
        let dp_du = Vector::new(-PI * 2.0 * p.y, PI * 2.0 * p.x, 0.0);
        let dp_dv = Vector::new(p.z * cos_phi, p.z * sin_phi, -self.radius * f32::sin(theta)) * PI;

        Some(DifferentialGeometry::with_normal(
            &p, &n, u, v, ray.time, &dp_du, &dp_dv, self,
        ))
    }
}

impl Boundable for Sphere {}
impl Sampleable for Sphere {}

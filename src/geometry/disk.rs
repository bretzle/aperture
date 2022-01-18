//! Defines a Disk type which implements the Geometry, Boundable and Sampleable traits
//! A disk with some inner and outer radius allowing it to
//! have a hole in the middle. The disk is oriented with the center
//! at the origin and the normal pointing along +Z.
//!
//! # Scene Usage Example
//! The disk requires two parameters, to specify the radius of the disk and the
//! radius of the hole cut out of the middle of it. Set the inner radius to 0 to
//! get a solid disk.
//!
//! ```json
//! "geometry": {
//!     "type": "disk",
//!     "radius": 4.0,
//!     "inner_radius": 1.0
//! }
/// ```
use std::f32;

use crate::geometry::{BBox, Boundable, DifferentialGeometry, Geometry, Sampleable};
use crate::linalg::{self, Normal, Point, Ray, Vector};
use crate::mc;

/// A disk with some inner and outer radius allowing it to
/// have a hole in the middle. The disk is oriented with the center
/// at the origin and the normal pointing along +Z.
#[derive(Clone, Copy)]
pub struct Disk {
    radius: f32,
    inner_radius: f32,
}

impl Disk {
    /// Create a new disk with some inner and outer radius
    pub fn new(radius: f32, inner_radius: f32) -> Self {
        Self {
            radius,
            inner_radius,
        }
    }
}

impl Geometry for Disk {
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry> {
        // The disk lies in the XY plane so if the ray doesn't cross this plane
        // there won't be any intersection
        if f32::abs(ray.d.z) == 0.0 {
            return None;
        }
        // We still treat the disk as an infinite XY plane for just a little longer
        // and here find the point where the ray crosses this plane
        let t = -ray.o.z / ray.d.z;
        if t < ray.min_t || t > ray.max_t {
            return None;
        }
        // We've hit the plane so now see if that hit is on the disk
        let p = ray.at(t);
        let dist_sqr = p.x * p.x + p.y * p.y;
        if dist_sqr > self.radius * self.radius || dist_sqr < self.inner_radius * self.inner_radius
        {
            return None;
        }
        let mut phi = f32::atan2(p.y, p.x);
        if phi < 0.0 {
            phi += f32::consts::PI * 2.0;
        }
        if phi > f32::consts::PI * 2.0 {
            return None;
        }
        ray.max_t = t;
        let hit_radius = f32::sqrt(dist_sqr);
        let u = phi / (2.0 * f32::consts::PI);
        let v = 1.0 - (hit_radius - self.inner_radius) / (self.radius - self.inner_radius);
        let dp_du = Vector::new(
            -f32::consts::PI * 2.0 * p.y,
            f32::consts::PI * 2.0 * p.x,
            0.0,
        );
        let dp_dv = ((self.inner_radius - self.radius) / hit_radius) * Vector::new(p.x, p.y, 0.0);
        Some(DifferentialGeometry::new(
            &p,
            &Normal::new(0.0, 0.0, 1.0),
            u,
            v,
            ray.time,
            &dp_du,
            &dp_dv,
            self,
        ))
	}
}

impl Boundable for Disk {
    fn bounds(&self, _: f32, _: f32) -> BBox {
        BBox::span(
            Point::new(-self.radius, -self.radius, -0.1),
            Point::new(self.radius, self.radius, 0.1),
        )
    }
}

impl Sampleable for Disk {
    fn sample_uniform(&self, samples: &(f32, f32)) -> (Point, Normal) {
        let disk_pos = mc::concentric_sample_disk(samples);
        let p = Point::new(disk_pos.0 * self.radius, disk_pos.1 * self.radius, 0.0);
        let n = Normal::new(0.0, 0.0, 1.0);
        (p, n)
    }
    fn sample(&self, _: &Point, samples: &(f32, f32)) -> (Point, Normal) {
        self.sample_uniform(samples)
    }
    fn surface_area(&self) -> f32 {
        f32::consts::PI * (self.radius * self.radius - self.inner_radius * self.inner_radius)
    }
    fn pdf(&self, p: &Point, w_i: &Vector) -> f32 {
        // Time doesn't matter here, we're already in the object's space so we're moving
        // with it so to speak
        let mut ray = Ray::segment(p, w_i, 0.001, f32::INFINITY, 0.0);
        match self.intersect(&mut ray) {
            Some(d) => {
                let w = -*w_i;
                let pdf = p.distance_sqr(&ray.at(ray.max_t))
                    / (f32::abs(linalg::dot(&d.n, &w)) * self.surface_area());
                if f32::is_finite(pdf) {
                    pdf
                } else {
                    0.0
                }
            }
            None => 0.0,
        }
    }
}

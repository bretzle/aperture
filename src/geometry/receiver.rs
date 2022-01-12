//! A receiver is an instance of geometry that does not emit any light

use crate::geometry::{BBox, Boundable, BoundableGeom, DifferentialGeometry};
use crate::linalg::{AnimatedTransform, Ray};
use crate::material::Material;
use std::sync::Arc;

/// An instance of geometry in the scene that only receives light
pub struct Receiver {
    /// The geometry that's being instanced.
    geom: Arc<dyn BoundableGeom + Send + Sync>,
    /// The material being used by this instance.
    pub material: Arc<dyn Material + Send + Sync>,
    /// The transform to world space
    transform: AnimatedTransform,
    /// Tag to identify the instance
    pub tag: String,
}

impl Receiver {
    /// Create a new instance of some geometry in the scene
    pub fn new(
        geom: Arc<dyn BoundableGeom + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
        transform: AnimatedTransform,
        tag: String,
    ) -> Self {
        Self {
            geom,
            material,
            transform,
            tag,
        }
    }
    /// Test the ray for intersection against this insance of geometry.
    /// returns Some(Intersection) if an intersection was found and None if not.
    /// If an intersection is found `ray.max_t` will be set accordingly
    pub fn intersect(&self, ray: &mut Ray) -> Option<(DifferentialGeometry, &dyn Material)> {
        let transform = self.transform.transform(ray.time);
        let mut local = transform.inv_mul_ray(ray);
        let mut dg = match self.geom.intersect(&mut local) {
            Some(dg) => dg,
            None => return None,
        };
        ray.max_t = local.max_t;
        dg.p = transform * dg.p;
        dg.n = transform * dg.n;
        dg.ng = transform * dg.ng;
        dg.dp_du = transform * dg.dp_du;
        dg.dp_dv = transform * dg.dp_dv;
        Some((dg, &*self.material))
    }
    /// Get the transform to place the receiver into world space
    pub fn get_transform(&self) -> &AnimatedTransform {
        &self.transform
    }
    /// Set the transform to place the receiver into world space
    pub fn set_transform(&mut self, transform: AnimatedTransform) {
        self.transform = transform;
    }
}

impl Boundable for Receiver {
    fn bounds(&self, start: f32, end: f32) -> BBox {
        self.transform
            .animation_bounds(&self.geom.bounds(start, end), start, end)
    }
}

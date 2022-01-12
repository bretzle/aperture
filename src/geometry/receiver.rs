use std::sync::Arc;

use crate::{
    material::Material,
    math::{AnimatedTransform, Ray},
};

use super::{differential_geometry::DifferentialGeometry, BoundableGeom};

pub struct Receiver {
    geom: Arc<dyn BoundableGeom + Send + Sync>,
    material: Arc<dyn Material + Send + Sync>,
    transform: AnimatedTransform,
    _tag: String,
}

impl Receiver {
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
            _tag: tag,
        }
    }

    pub fn intersect(
        &self,
        ray: &mut Ray,
    ) -> Option<(DifferentialGeometry, &(dyn Material + Send + Sync))> {
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
}

use std::sync::Arc;

use crate::{math::{AnimatedTransform, Ray}, material::Material};

use super::{BoundableGeom, differential_geometry::DifferentialGeometry};

pub struct Receiver<G, M> {
    geom: Arc<G>,
    material: Arc<M>,
    transform: AnimatedTransform,
    _tag: String,
}

impl<G, M> Receiver<G, M>
where
    G: BoundableGeom + Send + Sync,
    M: Material + Send + Sync,
{
    pub fn new(geom: Arc<G>, material: Arc<M>, transform: AnimatedTransform, tag: String) -> Self {
        Self {
            geom,
            material,
            transform,
            _tag: tag,
        }
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<(DifferentialGeometry, &M)> {
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

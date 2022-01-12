use std::sync::Arc;

use crate::{
    material::Material,
    math::{AnimatedTransform, Ray},
};

use super::{
    differential_geometry::DifferentialGeometry, intersection::Intersection, BoundableGeom,
};

pub enum Instance<G, M> {
    Receiver(Receiver<G, M>),
}

impl<G, M> Instance<G, M>
where
    G: BoundableGeom + Send + Sync,
    M: Material + Send + Sync,
{
    pub fn receiver(
        geom: Arc<G>,
        material: Arc<M>,
        transform: AnimatedTransform,
        tag: String,
    ) -> Self {
        Instance::Receiver(Receiver::new(geom, material, transform, tag))
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<Intersection<G, M>> {
        let hit = match *self {
            Instance::Receiver(ref r) => r.intersect(ray),
        };

        hit.map(|(dg, mat)| Intersection::new(dg, self, mat))
    }
}

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

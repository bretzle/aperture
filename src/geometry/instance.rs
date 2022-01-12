use super::{intersection::Intersection, receiver::Receiver, BoundableGeom};
use crate::{
    material::Material,
    math::{AnimatedTransform, Ray},
};
use std::sync::Arc;

pub enum Instance {
    Receiver(Receiver),
}

impl Instance {
    pub fn receiver(
        geom: Arc<dyn BoundableGeom + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
        transform: AnimatedTransform,
        tag: String,
    ) -> Self {
        Instance::Receiver(Receiver::new(geom, material, transform, tag))
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        let hit = match *self {
            Instance::Receiver(ref r) => r.intersect(ray),
        };

        hit.map(|(dg, mat)| Intersection::new(dg, self, mat))
    }
}

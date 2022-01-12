use super::{intersection::Intersection, receiver::Receiver, BoundableGeom};
use crate::{
    material::Material,
    math::{AnimatedTransform, Ray},
};
use std::sync::Arc;

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

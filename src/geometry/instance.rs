use std::sync::Arc;

use crate::{
    material::Material,
    math::{AnimatedTransform, Ray},
};

use super::{intersection::Intersection, BoundableGeom};

pub enum Instance<G, M> {
    Receiver(Receiver<G, M>),
}

impl<G, M> Instance<G, M> {
    pub fn receiver(
        geom: Arc<G>,
        material: Arc<M>,
        transform: AnimatedTransform,
        tag: String,
    ) -> Self
    where
        G: BoundableGeom + Send + Sync,
        M: Material + Send + Sync,
    {
        Instance::Receiver(Receiver::new(geom, material, transform, tag))
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<Intersection<G, M>> {
        todo!()
    }
}

pub struct Receiver<G, M> {
    geom: Arc<G>,
    material: Arc<M>,
    transform: AnimatedTransform,
    tag: String,
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
            tag,
        }
    }
}

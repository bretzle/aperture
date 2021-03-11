use crate::{
    geometry::Vector3f,
    interaction::{Interaction, SurfaceInteraction},
    material::TransportMode,
    spectrum::Spectrum,
};
use light_arena::Allocator;
use std::{fmt::Debug, sync::Arc};

pub trait Primitive: Debug + Send + Sync {
    fn area_light(&self) -> Option<Arc<dyn AreaLight>>;

    fn compute_scattering_functions<'a, 'b>(
        &self,
        isect: &mut SurfaceInteraction<'a, 'b>,
        mode: TransportMode,
        allow_multiple_lobes: bool,
        arena: &'b Allocator<'_>,
    );
}

pub trait AreaLight: Light {
    fn l(&self, si: &Interaction, w: &Vector3f) -> Spectrum;
}

pub trait Light: Debug + Send + Sync {}

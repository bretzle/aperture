use std::fmt::Debug;

use light_arena::Allocator;

use crate::interaction::SurfaceInteraction;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransportMode {
    RADIANCE,
    IMPORTANCE,
}

pub trait Material: Debug + Send + Sync {
    fn compute_scattering_functions<'a, 'b>(
        &self,
        isect: &mut SurfaceInteraction<'a, 'b>,
        mode: TransportMode,
        allow_multiple_lobes: bool,
        arena: &'b Allocator<'_>,
    );
}

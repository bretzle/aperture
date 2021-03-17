use crate::{
    geometry::{Bounds3f, Ray, Vector3f},
    interaction::{Interaction, SurfaceInteraction},
    material::{Material, TransportMode},
    shapes::Shape,
    spectrum::Spectrum,
    transform::Transform,
};
use light_arena::Allocator;
use std::{fmt::Debug, sync::Arc};

/// The bridge between geometry processing and shading subsystems of pbrt
pub trait Primitive: Debug + Send + Sync {
    /// Box that encloses the primitive's geometry in world space
    fn world_bounds(&self) -> Bounds3f;

    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool;

    /// The primitive's emission distribution.
    fn area_light(&self) -> Option<Arc<dyn AreaLight>>;

    /// The primitive's material
    /// if material is None the primitive only serves to delineate a volume of space
    /// Can test if two rays have intersected the same object by comparing material ptrs
    fn material(&self) -> Option<Arc<dyn Material>>;

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

#[derive(Debug)]
pub struct GeometricPrimitive {
    pub shape: Arc<dyn Shape>,
    pub area_light: Option<Arc<dyn AreaLight>>,
    pub material: Option<Arc<dyn Material>>,
}

impl Primitive for GeometricPrimitive {
    fn world_bounds(&self) -> Bounds3f {
        self.shape.world_bounds()
    }

    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        self.shape.intersect(ray).map(|(mut intersection, hit)| {
            intersection.primitive = Some(self);
            ray.t_max = hit;
            intersection
        })
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.shape.intersect_p(ray)
    }

    fn area_light(&self) -> Option<Arc<dyn AreaLight>> {
        self.area_light.clone()
    }

    fn material(&self) -> Option<Arc<dyn Material>> {
        self.material.clone()
    }

    fn compute_scattering_functions<'a, 'b>(
        &self,
        isect: &mut SurfaceInteraction<'a, 'b>,
        mode: TransportMode,
        allow_multiple_lobes: bool,
        arena: &'b Allocator<'_>,
    ) {
        if let Some(material) = &self.material {
            material.compute_scattering_functions(isect, mode, allow_multiple_lobes, arena);
        }
    }
}

#[derive(Debug)]
pub struct TransformedPrimitive {
    pub primitive: Arc<dyn Primitive>,
    pub primitive_to_world: Transform, // TODO should be animated
}

impl Primitive for TransformedPrimitive {
    fn world_bounds(&self) -> Bounds3f {
        &self.primitive_to_world * &self.primitive.world_bounds()
    }

    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        let mut r = self.primitive_to_world.inverse() * *ray;
        self.primitive.intersect(&mut r).map(|intersection| {
            ray.t_max = r.t_max;
            intersection.transform(&self.primitive_to_world)
        })
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let r = self.primitive_to_world.inverse() * *ray;
        self.primitive.intersect_p(&r)
    }

    fn area_light(&self) -> Option<Arc<dyn AreaLight>> {
        None
    }

    fn material(&self) -> Option<Arc<dyn Material>> {
        None
    }

    fn compute_scattering_functions<'a, 'b>(
        &self,
        _: &mut SurfaceInteraction<'a, 'b>,
        _: TransportMode,
        _: bool,
        _: &'b Allocator<'_>,
    ) {
        panic!("Should never be called!")
    }
}

use super::Shape;
use crate::{
    bounds::Bounds3f,
    interaction::{Interaction, SurfaceInteraction},
    paramset::ParamSet,
    ray::Ray,
    transform::Transform,
};
use maths::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Paraboloid {
    object_to_world: Transform,
    world_to_object: Transform,
    radius: f32,
    z_min: f32,
    z_max: f32,
    phi_max: f32,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,
}

impl Paraboloid {
    pub fn new(
        o2w: Transform,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
        reverse_orientation: bool,
    ) -> Self {
        let transform_swaps_handedness = o2w.swaps_handedness();
        Self {
            world_to_object: o2w.inverse(),
            object_to_world: o2w,
            radius,
            z_min: z_min.min(z_max),
            z_max: z_max.max(z_min),
            phi_max: clamp(phi_max, 0.0, 360.0),
            reverse_orientation,
            transform_swaps_handedness,
        }
    }

    pub fn create(o2w: &Transform, reverse_orientation: bool, params: &ParamSet) -> Arc<dyn Shape> {
        let radius = params.find_one_float("radius", 1.0);
        let z_min = params.find_one_float("zmin", 1.0);
        let z_max = params.find_one_float("zmax", 1.0);
        let phi_max = params.find_one_float("phimax", 360.0);

        Arc::new(Paraboloid::new(
            o2w.clone(),
            radius,
            z_min,
            z_max,
            phi_max,
            reverse_orientation,
        ))
    }
}

impl Shape for Paraboloid {
    fn intersect(&self, _ray: &Ray) -> Option<(SurfaceInteraction, f32)> {
        todo!()
    }

    fn area(&self) -> f32 {
        let radius2 = self.radius * self.radius;
        let k = 4.0 * self.z_max / radius2;
        (radius2 * radius2 * self.phi_max / (12.0 * self.z_max * self.z_max))
            * ((k * self.z_max + 1.0).powf(1.5) - (k * self.z_min + 1.0).powf(1.5))
    }

    fn object_bounds(&self) -> Bounds3f {
        Bounds3f::from_points(
            &Point3f::new(-self.radius, -self.radius, self.z_min),
            &Point3f::new(self.radius, self.radius, self.z_max),
        )
    }

    fn world_bounds(&self) -> Bounds3f {
        todo!()
    }

    fn sample(&self, _u: Point2f) -> (Interaction, f32) {
        todo!()
    }

    fn reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    fn transform_swaps_handedness(&self) -> bool {
        self.transform_swaps_handedness
    }
}

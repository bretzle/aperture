use super::Shape;
use crate::{bounds::Bounds3f, interaction::{Interaction, SurfaceInteraction}, paramset::ParamSet, ray::Ray, transform::Transform};
use maths::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Cone {
    object_to_world: Transform,
    world_to_object: Transform,
    radius: f32,
    height: f32,
    phi_max: f32,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,
}

impl Cone {
    pub fn new(
        o2w: Transform,
        radius: f32,
        height: f32,
        phi_max: f32,
        reverse_orientation: bool,
    ) -> Self {
        let transform_swaps_handedness = o2w.swaps_handedness();
        Self {
            world_to_object: o2w.inverse(),
            object_to_world: o2w,
            radius,
            height,
            phi_max: clamp(phi_max, 0.0, 360.0),
            reverse_orientation,
            transform_swaps_handedness,
        }
    }

    pub fn create(o2w: &Transform, reverse_orientation: bool, params: &ParamSet) -> Arc<dyn Shape> {
        let height = params.find_one_float("height", 0.0);
        let radius = params.find_one_float("radius", 1.0);
        let phi_max = params.find_one_float("phimax", 360.0);

        Arc::new(Cone::new(
            o2w.clone(),
            radius,
            height,
            phi_max,
            reverse_orientation,
        ))
    }
}

impl Shape for Cone {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, f32)> {
        todo!()
    }

    fn area(&self) -> f32 {
        self.radius * (self.height * self.height + self.radius * self.radius).sqrt() * self.phi_max
            / 2.0
    }

    fn object_bounds(&self) -> Bounds3f {
        Bounds3f::from_points(
            &Point3f::new(-self.radius, -self.radius, 0.0),
            &Point3f::new(self.radius, self.radius, self.height),
        )
    }

    fn world_bounds(&self) -> Bounds3f {
        todo!()
    }

    fn sample(&self, u: Point2f) -> (Interaction, f32) {
        unimplemented!()
    }

    fn reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    fn transform_swaps_handedness(&self) -> bool {
        self.transform_swaps_handedness
    }
}

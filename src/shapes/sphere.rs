use super::Material;
use crate::{
    interaction::{Interaction, InteractionBase, SurfaceInteraction},
    math::{clamp, Angle, Bounds3, Point2, Point3, Ray, Vector3},
    transform::Transform,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub radius: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub theta_min: f32,
    pub theta_max: f32,
    pub phi_max: f32,
    // inherited from class Shape (see shape.h)
    pub object_to_world: Transform,
    pub world_to_object: Transform,
    pub reverse_orientation: bool,
    pub transform_swaps_handedness: bool,
    pub material: Option<Arc<Material>>,
}

impl Default for Sphere {
    fn default() -> Self {
        let object_to_world: Transform = Transform::default();
        Self {
            // Shape
            object_to_world,
            world_to_object: Transform::default(),
            reverse_orientation: false,
            transform_swaps_handedness: object_to_world.swaps_handedness(),
            // Sphere
            radius: 1.0,
            z_min: -1.0,
            z_max: 1.0,
            theta_min: (-1.0 as f32).acos(),
            theta_max: (1.0 as f32).acos(),
            phi_max: Angle::radians(360.0),
            material: None,
        }
    }
}

impl Sphere {
    pub fn new(
        object_to_world: Transform,
        world_to_object: Transform,
        reverse_orientation: bool,
        radius: f32,
        z_min: f32,
        z_max: f32,
        phi_max: f32,
    ) -> Self {
        Self {
            // Shape
            object_to_world,
            world_to_object,
            reverse_orientation,
            transform_swaps_handedness: object_to_world.swaps_handedness(),
            // Sphere
            radius,
            z_min: clamp(z_min.min(z_max), -radius, radius),
            z_max: clamp(z_min.max(z_max), -radius, radius),
            theta_min: clamp(z_min.min(z_max) / radius, -1.0, 1.0).acos(),
            theta_max: clamp(z_min.max(z_max) / radius, -1.0, 1.0).acos(),
            phi_max: Angle::radians(clamp(phi_max, 0.0, 360.0)),
            material: None,
        }
    }

    // Shape
    pub fn object_bound(&self) -> Bounds3<f32> {
        Bounds3 {
            p_min: Point3 {
                x: -self.radius,
                y: -self.radius,
                z: self.z_min,
            },
            p_max: Point3 {
                x: self.radius,
                y: self.radius,
                z: self.z_max,
            },
        }
    }
    pub fn world_bound(&self) -> Bounds3<f32> {
        self.object_to_world.transform_bounds(self.object_bound())
    }

    pub fn intersect(&self, r: &Ray, t_hit: &mut f32, isect: &mut SurfaceInteraction) -> bool {
        todo!()
    }

    pub fn intersect_p(&self, r: &Ray) -> bool {
        todo!()
    }

    pub fn get_reverse_orientation(&self) -> bool {
        self.reverse_orientation
    }

    pub fn get_transform_swaps_handedness(&self) -> bool {
        self.transform_swaps_handedness
    }

    pub fn get_object_to_world(&self) -> Transform {
        self.object_to_world
    }

    pub fn area(&self) -> f32 {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }

    pub fn sample(&self, u: Point2<f32>, pdf: &mut f32) -> InteractionBase {
        todo!()
    }

    pub fn sample_with_ref_point(
        &self,
        iref: &InteractionBase,
        u: Point2<f32>,
        pdf: &mut f32,
    ) -> InteractionBase {
        todo!()
    }

    pub fn pdf_with_ref_point(&self, iref: &dyn Interaction, wi: &Vector3<f32>) -> f32 {
        todo!()
    }
}

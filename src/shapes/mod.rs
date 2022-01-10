mod sphere;

pub use sphere::*;

use crate::{
    interaction::{Interaction, InteractionBase, SurfaceInteraction},
    math::{Bounds3, Point2, Ray, Vector3},
    transform::Transform,
};

// TODO: get rid of this when actual materials are implimented
struct Material;

pub enum Shape {
    // Curve(Curve),
    // Cylinder(Cylinder),
    // Disk(Disk),
    Sphere(Sphere),
    // Triangle(Triangle),
}

impl Shape {
    pub fn object_bound(&self) -> Bounds3<f32> {
        match self {
            // Shape::Curve(shape) => shape.object_bound(),
            // Shape::Cylinder(shape) => shape.object_bound(),
            // Shape::Disk(shape) => shape.object_bound(),
            Shape::Sphere(shape) => shape.object_bound(),
            // Shape::Triangle(shape) => shape.object_bound(),
        }
    }

    pub fn world_bound(&self) -> Bounds3<f32> {
        match self {
            // Shape::Curve(shape) => shape.world_bound(),
            // Shape::Cylinder(shape) => shape.world_bound(),
            // Shape::Disk(shape) => shape.world_bound(),
            Shape::Sphere(shape) => shape.world_bound(),
            // Shape::Triangle(shape) => shape.world_bound(),
        }
    }

    pub fn intersect(&self, r: &Ray, t_hit: &mut f32, isect: &mut SurfaceInteraction) -> bool {
        match self {
            // Shape::Curve(shape) => shape.intersect(r, t_hit, isect),
            // Shape::Cylinder(shape) => shape.intersect(r, t_hit, isect),
            // Shape::Disk(shape) => shape.intersect(r, t_hit, isect),
            Shape::Sphere(shape) => shape.intersect(r, t_hit, isect),
            // Shape::Triangle(shape) => shape.intersect(r, t_hit, isect),
        }
    }

    pub fn intersect_p(&self, r: &Ray) -> bool {
        match self {
            // Shape::Curve(shape) => shape.intersect_p(r),
            // Shape::Cylinder(shape) => shape.intersect_p(r),
            // Shape::Disk(shape) => shape.intersect_p(r),
            Shape::Sphere(shape) => shape.intersect_p(r),
            // Shape::Triangle(shape) => shape.intersect_p(r),
        }
    }

    pub fn get_reverse_orientation(&self) -> bool {
        match self {
            // Shape::Curve(shape) => shape.get_reverse_orientation(),
            // Shape::Cylinder(shape) => shape.get_reverse_orientation(),
            // Shape::Disk(shape) => shape.get_reverse_orientation(),
            Shape::Sphere(shape) => shape.get_reverse_orientation(),
            // Shape::Triangle(shape) => shape.get_reverse_orientation(),
        }
    }

    pub fn get_transform_swaps_handedness(&self) -> bool {
        match self {
            // Shape::Curve(shape) => shape.get_transform_swaps_handedness(),
            // Shape::Cylinder(shape) => shape.get_transform_swaps_handedness(),
            // Shape::Disk(shape) => shape.get_transform_swaps_handedness(),
            Shape::Sphere(shape) => shape.get_transform_swaps_handedness(),
            // Shape::Triangle(shape) => shape.get_transform_swaps_handedness(),
        }
    }

    pub fn get_object_to_world(&self) -> Transform {
        match self {
            // Shape::Curve(shape) => shape.get_object_to_world(),
            // Shape::Cylinder(shape) => shape.get_object_to_world(),
            // Shape::Disk(shape) => shape.get_object_to_world(),
            Shape::Sphere(shape) => shape.get_object_to_world(),
            // Shape::Triangle(shape) => shape.get_object_to_world(),
        }
    }

    pub fn area(&self) -> f32 {
        match self {
            // Shape::Curve(shape) => shape.area(),
            // Shape::Cylinder(shape) => shape.area(),
            // Shape::Disk(shape) => shape.area(),
            Shape::Sphere(shape) => shape.area(),
            // Shape::Triangle(shape) => shape.area(),
        }
    }

    pub fn sample(&self, u: Point2<f32>, pdf: &mut f32) -> InteractionBase {
        match self {
            // Shape::Curve(shape) => shape.sample(u, pdf),
            // Shape::Cylinder(shape) => shape.sample(u, pdf),
            // Shape::Disk(shape) => shape.sample(u, pdf),
            Shape::Sphere(shape) => shape.sample(u, pdf),
            // Shape::Triangle(shape) => shape.sample(u, pdf),
        }
    }

    pub fn pdf(&self, _iref: &InteractionBase) -> f32 {
        1.0 / self.area()
    }

    pub fn sample_with_ref_point(
        &self,
        iref: &InteractionBase,
        u: Point2<f32>,
        pdf: &mut f32,
    ) -> InteractionBase {
        match self {
            // Shape::Curve(shape) => shape.sample_with_ref_point(iref, u, pdf),
            // Shape::Cylinder(shape) => shape.sample_with_ref_point(iref, u, pdf),
            // Shape::Disk(shape) => shape.sample_with_ref_point(iref, u, pdf),
            Shape::Sphere(shape) => shape.sample_with_ref_point(iref, u, pdf),
            // Shape::Triangle(shape) => shape.sample_with_ref_point(iref, u, pdf),
        }
    }

    pub fn pdf_with_ref_point(&self, iref: &dyn Interaction, wi: &Vector3<f32>) -> f32 {
        match self {
            // Shape::Curve(shape) => shape.pdf_with_ref_point(iref, wi),
            // Shape::Cylinder(shape) => shape.pdf_with_ref_point(iref, wi),
            // Shape::Disk(shape) => shape.pdf_with_ref_point(iref, wi),
            Shape::Sphere(shape) => shape.pdf_with_ref_point(iref, wi),
            // Shape::Triangle(shape) => shape.pdf_with_ref_point(iref, wi),
        }
    }
}

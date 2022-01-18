//! The geometry module defines the Geometry trait implemented by
//! the various geometry in the ray tracer and provides some standard
//! geometry for rendering
//!
//! # Scene Usage Example
//! All geometry will appear within an object specification and requires the type
//! of geometry being specified along with any parameters for that geometry.
//!
//! An instance has a geometry along with additional information like a material
//! and transformation to place it in the world, see the instance module for more.
//!
//! ```json
//! "objects": [
//!     {
//!          "type": "The_Instance_Type",
//!          ...
//!          "geometry": {
//!              "type": "The_Geometry_Type",
//!              ...
//!          }
//!     },
//!     ...
//! ]
//! ```

use crate::linalg::{Normal, Point, Ray, Vector};

pub use self::{
    animated_mesh::AnimatedMesh, bbox::BBox, bvh::BVH, differential_geometry::DifferentialGeometry,
    disk::Disk, emitter::Emitter, instance::Instance, intersection::Intersection, mesh::Mesh,
    receiver::Receiver, rectangle::Rectangle, sphere::Sphere,
};
use self::{animated_mesh::AnimatedTriangle, mesh::Triangle};

pub mod animated_mesh;
pub mod bbox;
pub mod bvh;
pub mod differential_geometry;
pub mod disk;
pub mod emitter;
pub mod instance;
pub mod intersection;
pub mod mesh;
pub mod receiver;
pub mod rectangle;
pub mod sphere;

/// Trait implemented by geometric primitives
#[enum_dispatch(BoundableGeometry, SampleableGeometry)]
pub trait Geometry {
    /// Test a ray for intersection with the geometry.
    /// The ray should have been previously transformed into the geometry's
    /// object space otherwise the test will be incorrect.
    /// Returns the differential geometry containing the hit information if the
    /// ray hit the object and set's the ray's `max_t` member accordingly
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry>;
}

/// Trait implemented by scene objects that can report an AABB describing their bounds
#[enum_dispatch(BoundableGeometry, SampleableGeometry)]
pub trait Boundable {
    /// Get an AABB reporting the object's bounds over the time period
    /// The default implementation assumes the object isn't animated and
    /// simply returns its bounds. This is kind of a hack to use
    /// the BVH for animated geomtry (instances) and non-animated geometry (triangles).
    fn bounds(&self, start: f32, end: f32) -> BBox;
    /// Have the object recompute its bounds for the time range. In the case
    /// of deforming geometry this can rebuild acceleration structures for example.
    fn update_deformation(&mut self, _start: f32, _end: f32) {}
}

/// Trait implemented by geometry that can sample a point on its surface
#[enum_dispatch(SampleableGeometry)]
pub trait Sampleable {
    /// Uniformly sample a position and normal on the surface using the samples passed
    fn sample_uniform(&self, samples: &(f32, f32)) -> (Point, Normal);
    /// Sample the object using the probability density of the solid angle
    /// from `p` to the sampled point on the surface.
    /// Returns the sampled point and the surface normal at that point
    fn sample(&self, p: &Point, samples: &(f32, f32)) -> (Point, Normal);
    /// Return the surface area of the shape
    fn surface_area(&self) -> f32;
    /// Compute the PDF that the ray from `p` with direction `w_i` intersects
    /// the shape
    fn pdf(&self, p: &Point, w_i: &Vector) -> f32;
}

pub trait BoundableGeom: Geometry + Boundable {}
impl<T: ?Sized> BoundableGeom for T where T: Geometry + Boundable {}

#[enum_dispatch]
pub enum BoundableGeometry {
    AnimatedMesh,
    AnimatedTriangle,
    Disk,
    Mesh,
    Triangle,
    Rectangle,
    Sphere,
}

pub trait SampleableGeom: Geometry + Boundable + Sampleable {}
impl<T: ?Sized> SampleableGeom for T where T: Geometry + Boundable + Sampleable {}

#[enum_dispatch]
pub enum SampleableGeometry {
    Disk,
    Rectangle,
    Sphere,
}

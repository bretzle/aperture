//! Defines the Intersection type which stores information about
//! a full intersection, eg. hit info about the geometry and instance
//! that was intersected

use crate::{
    geometry::{DifferentialGeometry, Instance},
    material::Materials,
};

/// Stores information about an intersection that occured with some instance
/// of geometry in the scene
#[derive(Clone, Copy)]
pub struct Intersection<'a, 'b> {
    /// The differential geometry holding information about the piece of geometry
    /// that was hit
    pub dg: DifferentialGeometry<'a>,
    /// The instance of geometry that was hit
    pub instance: &'b Instance,
    /// The material of the instance that was hit
    pub material: &'b Materials,
}

impl<'a, 'b> Intersection<'a, 'b> {
    /// Construct the Intersection from a potential hit stored in a
    /// Option<DifferentialGeometry>. Returns None if `dg` is None
    /// or if the instance member of `dg` is None
    pub fn new(
        dg: DifferentialGeometry<'a>,
        instance: &'b Instance,
        material: &'b Materials,
    ) -> Self {
        Self {
            dg,
            instance,
            material,
        }
    }
}

use crate::material::Material;

use super::{differential_geometry::DifferentialGeometry, instance::Instance};

#[derive(Clone, Copy)]
pub struct Intersection<'a, 'b, G, M> {
    /// The differential geometry holding information about the piece of geometry
    /// that was hit
    pub dg: DifferentialGeometry<'a>,
    /// The instance of geometry that was hit
    pub instance: &'b Instance<G, M>,
    /// The material of the instance that was hit
    pub material: &'b dyn Material,
}

impl<'a, 'b, G, M> Intersection<'a, 'b, G, M> {
    /// Construct the Intersection from a potential hit stored in a
    /// Option<DifferentialGeometry>. Returns None if `dg` is None
    /// or if the instance member of `dg` is None
    pub fn new(
        dg: DifferentialGeometry<'a>,
        inst: &'b Instance<G, M>,
        mat: &'b dyn Material,
    ) -> Self {
        Self {
            dg,
            instance: inst,
            material: mat,
        }
    }
}

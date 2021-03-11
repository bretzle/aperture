mod bxdf;

use self::bxdf::BxDF;
use crate::geometry::*;

/// Represents the Bidirectional Scattering Distribution Function.
/// It represents the properties of a material at a given point.
pub struct BSDF<'a> {
    /// Index of refraction of the surface
    pub eta: f32,
    /// Shading normal (i.e. potentially affected by bump-mapping)
    ns: Normal3f,
    /// Geometry normal
    ng: Normal3f,
    ss: Vector3f,
    ts: Vector3f,
    pub bxdfs: &'a [&'a dyn BxDF],
}

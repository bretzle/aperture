use crate::math::{Normal, Point, Vector};

use super::Geometry;

#[derive(Clone, Copy)]
pub struct DifferentialGeometry<'a> {
    /// The hit point
    pub p: Point,
    /// The shading normal
    pub n: Normal,
    /// The geometry normal
    pub ng: Normal,
    /// Surface parameterization u, v for texture mapping
    pub u: f32,
    pub v: f32,
    /// The intersection time
    pub time: f32,
    /// Derivative of the point with respect to the u parameterization coord of the surface
    pub dp_du: Vector,
    /// Derivative of the point with respect to the v parameterization coord of the surface
    pub dp_dv: Vector,
    /// The geometry that was hit
    pub geom: &'a (dyn Geometry + 'a),
}

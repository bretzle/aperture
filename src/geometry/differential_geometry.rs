//! Defines the `DifferentialGeometry` type which is used to pass information
//! about the hit piece of geometry back from the intersection to the shading

use crate::{
    geometry::Geometry,
    linalg::{self, Normal, Point, Vector},
};

/// Stores information about a hit piece of geometry of some object in the scene
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

impl<'a> DifferentialGeometry<'a> {
    /// Setup the differential geometry. Note that the normal will be computed
    /// using cross(dp_du, dp_dv)
    pub fn new(
        p: &Point,
        ng: &Normal,
        u: f32,
        v: f32,
        time: f32,
        dp_du: &Vector,
        dp_dv: &Vector,
        geom: &'a (dyn Geometry + 'a),
    ) -> Self {
        let n = linalg::cross(dp_du, dp_dv).normalized();
        Self {
            p: *p,
            n: Normal::new(n.x, n.y, n.z),
            ng: ng.normalized(),
            u,
            v,
            time,
            dp_du: *dp_du,
            dp_dv: *dp_dv,
            geom,
        }
    }
    /// Setup the differential geometry using the normal passed for the surface normal
    pub fn with_normal(
        p: &Point,
        n: &Normal,
        u: f32,
        v: f32,
        time: f32,
        dp_du: &Vector,
        dp_dv: &Vector,
        geom: &'a (dyn Geometry + 'a),
    ) -> Self {
        let nn = n.normalized();
        Self {
            p: *p,
            n: nn,
            ng: nn,
            u,
            v,
            time,
            dp_du: *dp_du,
            dp_dv: *dp_dv,
            geom,
        }
    }
}

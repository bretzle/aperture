use crate::math::{self, Normal, Point, Vector};

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
    pub geom: &'a (dyn Geometry + 'a), // TODO: dont use dyn
}

impl<'a> DifferentialGeometry<'a> {
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
        let n = math::cross(dp_du, dp_dv).normalize();
        Self {
            p: *p,
            n: Normal::new(n.x, n.y, n.z),
            ng: ng.normalize(),
            u,
            v,
            time,
            dp_du: *dp_du,
            dp_dv: *dp_dv,
            geom,
        }
    }

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
        let nn = n.normalize();
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

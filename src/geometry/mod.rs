use self::differential_geometry::DifferentialGeometry;
use crate::math::Ray;

pub mod differential_geometry;
pub mod instance;
pub mod intersection;

pub trait Geometry {
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry>;
}

pub trait Boundable {}

pub trait Sampleable {}

pub trait BoundableGeom: Geometry + Boundable {}
impl<T: ?Sized> BoundableGeom for T where T: Geometry + Boundable {}

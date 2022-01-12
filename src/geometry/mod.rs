use crate::math::Ray;

pub use self::differential_geometry::DifferentialGeometry;
pub use self::instance::Instance;
pub use self::intersection::Intersection;
pub use self::receiver::Receiver;
pub use self::sphere::Sphere;

mod differential_geometry;
mod instance;
mod intersection;
mod receiver;
mod sphere;

pub trait Geometry {
    fn intersect(&self, ray: &mut Ray) -> Option<DifferentialGeometry>;
}

pub trait Boundable {}

pub trait Sampleable {}

pub trait BoundableGeom: Geometry + Boundable {}
impl<T: ?Sized> BoundableGeom for T where T: Geometry + Boundable {}

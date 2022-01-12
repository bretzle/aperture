pub mod instance;
pub mod intersection;
pub mod differential_geometry;

pub trait Geometry {}

pub trait Boundable {}

pub trait Sampleable {}

pub trait BoundableGeom: Geometry + Boundable {}
impl<T: ?Sized> BoundableGeom for T where T: Geometry + Boundable {}

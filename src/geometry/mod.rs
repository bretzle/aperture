mod bounds;
mod matrix;
mod normal;
mod point;
mod ray;
mod vector;

use self::bounds::*;
pub use self::matrix::*;
use self::normal::*;
use self::point::*;
use self::vector::*;

pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;
pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;
pub type Normal3f = Normal3<f32>;
pub type Bounds3f = Bounds3<f32>;
pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<f32>;

#[macro_export]
macro_rules! matrix {
	( $( $( $val:expr ),+ );* ; ) => {
		$crate::geometry::Matrix::new([ $( [$( $val as f32 ),+] ),* ]);
	};
}

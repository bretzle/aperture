mod bounds;
mod matrix;
mod normal;
mod point;
mod ray;
mod vector;

use crate::utils::{next_float_down, next_float_up};

use self::bounds::*;
pub use self::matrix::*;
use self::normal::*;
use self::point::*;
pub use self::ray::Ray;
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

#[inline]
pub fn offset_ray_origin(p: &Point3f, p_error: &Vector3f, n: &Normal3f, w: &Vector3f) -> Point3f {
    let d = n.abs().dot(p_error);
    let mut offset = d * Vector3f::from(*n);
    if w.dotn(n) < 0.0 {
        offset = -offset;
    }
    let mut po = *p + offset;
    // Round offset point `po` away from `p`
    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = next_float_up(po[i]);
        } else if offset[i] < 0.0 {
            po[i] = next_float_down(po[i]);
        }
    }

    po
}

#[inline]
pub fn face_forward_n(v1: &Normal3f, v2: &Normal3f) -> Normal3f {
    if v1.dotn(v2) < 0.0 {
        -(*v1)
    } else {
        *v1
    }
}

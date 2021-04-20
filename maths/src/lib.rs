#![cfg_attr(not(test), no_std)]
#![allow(clippy::excessive_precision)]
#![feature(const_fn_floating_point_arithmetic)]

pub use crate::{
	cmatrix::Matrix,
    normal::Normal3,
    point::{Point2, Point3},
    vector::{Vector2, Vector3},
};
use core::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};
use num::{Num, One, Signed};


pub mod cmatrix;
mod macros;
// mod matrix;
mod normal;
mod point;
mod vector;

pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;
pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;
pub type Normal3f = Normal3<f32>;

pub const MACHINE_EPSILON: f32 = f32::EPSILON * 0.5;
pub const INV_2_PI: f32 = 0.15915494309189533577;
pub const ONE_MINUS_EPSILON: f32 = 0.99999994f32;

#[macro_export]
macro_rules! matrix {
	// ( $( $( $val:expr ),+ );* ; ) => {
	// 	$crate::Matrix::new([ $( [$( $val as f32 ),+] ),* ]);
	// };
	( $( $( $val:expr ),+ );* ; ) => {
		$crate::cmatrix::Matrix::new([ $( [$( $val as f32 ),+] ),* ]);
	};
}

/// Return the dimension index (0, 1 or 2) that contains the largest component.
pub fn max_dimension<T>(v: &Vector3<T>) -> usize
where
    T: Num + PartialOrd,
{
    if v.x > v.y {
        if v.x > v.z {
            0
        } else {
            2
        }
    } else if v.y > v.z {
        1
    } else {
        2
    }
}

pub fn max_component(v: &Vector3f) -> f32 {
    f32::max(v.x, f32::max(v.y, v.z))
}

/// Permute the components of this vector based on the given indices for x, y
/// and z.
pub fn permute_v<T>(v: &Vector3<T>, x: usize, y: usize, z: usize) -> Vector3<T>
where
    T: Num + Copy,
{
    Vector3::new(v[x], v[y], v[z])
}

/// Permute the components of this point based on the given indices for x, y and
/// z.
pub fn permute_p<T>(v: &Point3<T>, x: usize, y: usize, z: usize) -> Point3<T>
where
    T: Num + Signed + Copy,
{
    Point3::new(v[x], v[y], v[z])
}

// Common geometric functions
#[inline]
pub const fn cos_theta(w: &Vector3f) -> f32 {
    w.z
}

#[inline]
pub fn cos2_theta(w: &Vector3f) -> f32 {
    w.z * w.z
}

#[inline]
pub fn abs_cos_theta(w: &Vector3f) -> f32 {
    w.z.abs()
}

#[inline]
pub fn sin2_theta(w: &Vector3f) -> f32 {
    (1.0 - cos2_theta(w)).max(0.0)
}

#[inline]
pub fn sin_theta(w: &Vector3f) -> f32 {
    sin2_theta(w).sqrt()
}

#[inline]
pub fn tan_theta(w: &Vector3f) -> f32 {
    sin_theta(w) / cos_theta(w)
}

#[inline]
pub fn tan2_theta(w: &Vector3f) -> f32 {
    sin2_theta(w) / cos2_theta(w)
}

#[inline]
pub fn cos_phi(w: &Vector3f) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        1.0
    } else {
        clamp(w.x / sin_theta, -1.0, 1.0)
    }
}

#[inline]
pub fn sin_phi(w: &Vector3f) -> f32 {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        0.0
    } else {
        clamp(w.y / sin_theta, -1.0, 1.0)
    }
}

#[inline]
pub fn cos2_phi(w: &Vector3f) -> f32 {
    cos_phi(w) * cos_phi(w)
}

#[inline]
pub fn sin2_phi(w: &Vector3f) -> f32 {
    sin_phi(w) * sin_phi(w)
}

#[inline]
pub fn cos_d_phi(wa: &Vector3f, wb: &Vector3f) -> f32 {
    let waxy = wa.x * wa.x + wa.y * wa.y;
    let wbxy = wb.x * wb.x + wb.y * wb.y;

    if waxy == 0.0 || wbxy == 0.0 {
        return 1.0;
    }

    clamp((wa.x * wb.x + wa.y * wb.y) / (waxy * wbxy).sqrt(), -1.0, 1.0)
}

#[inline]
pub fn same_hemisphere(w: &Vector3f, wp: &Vector3f) -> bool {
    w.z * wp.z > 0.0
}

#[inline]
pub fn spherical_theta(v: &Vector3f) -> f32 {
    clamp(v.z, -1.0, 1.0).acos()
}

#[inline]
pub fn spherical_phi(v: &Vector3f) -> f32 {
    let p = v.y.atan2(v.x);
    if p < 0.0 {
        p + 2.0 * PI
    } else {
        p
    }
}

#[inline]
pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector3f {
    Vector3f::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

#[inline]
pub fn spherical_direction_vec(
    sin_theta: f32,
    cos_theta: f32,
    phi: f32,
    x: &Vector3f,
    y: &Vector3f,
    z: &Vector3f,
) -> Vector3f {
    sin_theta * phi.cos() * *x + sin_theta * phi.sin() * *y + cos_theta * *z
}

#[inline]
pub fn face_forward(v1: &Vector3f, v2: &Vector3f) -> Vector3f {
    if v1.dot(v2) < 0.0 {
        -(*v1)
    } else {
        *v1
    }
}

#[inline]
pub fn face_forward_n(v1: &Normal3f, v2: &Normal3f) -> Normal3f {
    if v1.dotn(v2) < 0.0 {
        -(*v1)
    } else {
        *v1
    }
}

/// Polynomial approximation of the inverse Gauss error function
#[inline]
pub fn erf_inv(x: f32) -> f32 {
    let x = clamp(x, -0.99999, 0.99999);
    let mut w = -((1.0 - x) * (1.0 + x)).ln();
    let mut p;
    if w < 5.0 {
        w -= 2.5;
        p = 2.81022636E-08;
        p = 3.43273939E-07 + p * w;
        p = -3.5233877E-06 + p * w;
        p = -4.39150654E-06 + p * w;
        p = 0.00021858087 + p * w;
        p = -0.00125372503 + p * w;
        p = -0.00417768164 + p * w;
        p = 0.246640727 + p * w;
        p = 1.50140941 + p * w;
    } else {
        w = w.sqrt() - 3.0;
        p = -0.000200214257;
        p = 0.000100950558 + p * w;
        p = 0.00134934322 + p * w;
        p = -0.00367342844 + p * w;
        p = 0.00573950773 + p * w;
        p = -0.0076224613 + p * w;
        p = 0.00943887047 + p * w;
        p = 1.00167406 + p * w;
        p = 2.83297682 + p * w;
    }

    p * x
}

/// Polynomial approximation of the Gauss error function.
///
/// See [this link](https://en.wikipedia.org/wiki/Error_function)
pub fn erf(x: f32) -> f32 {
    // constants
    let a1: f32 = 0.254829592;
    let a2: f32 = -0.284496736;
    let a3: f32 = 1.421413741;
    let a4: f32 = -1.453152027;
    let a5: f32 = 1.061405429;
    let p: f32 = 0.3275911;

    // Save the sign of x
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    // A&S formula 7.1.26
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
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

pub fn distance_squared(p1: &Point3f, p2: &Point3f) -> f32 {
    (*p2 - *p1).length_sq()
}

pub fn distance(p1: &Point3f, p2: &Point3f) -> f32 {
    (*p2 - *p1).length()
}

pub fn clamp<T>(val: T, low: T, high: T) -> T
where
    T: PartialOrd + Copy,
{
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

#[inline]
pub fn next_float_up(v: f32) -> f32 {
    let mut v = v;
    if v.is_infinite() && v > 0.0 {
        return v;
    }

    if v == -0.0 {
        v = 0.0;
    }
    let mut ui = v.to_bits();
    if v >= 0.0 {
        ui += 1;
    } else {
        ui -= 1;
    }
    f32::from_bits(ui)
}

#[inline]
pub fn next_float_down(v: f32) -> f32 {
    let mut v = v;
    if v.is_infinite() && v < 0.0 {
        return v;
    }

    if v == 0.0 {
        v = -0.0;
    }
    let mut ui = v.to_bits();
    if v > 0.0 {
        ui -= 1;
    } else {
        ui += 1;
    }
    f32::from_bits(ui)
}

/// Version of min() that works on `PartialOrd`, so it works for both u32 and
/// f32.
pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a.lt(&b) {
        a
    } else {
        b
    }
}

/// Version of max() that works on `PartialOrd`, so it works for both u32 and
/// f32.
pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a.gt(&b) {
        a
    } else {
        b
    }
}

/// Linear interpolation between 2 values.
///
/// This version should be generic enough to linearly interpolate between 2
/// Spectrums using an f32 parameter.
pub fn lerp<S, T>(t: S, a: T, b: T) -> T
where
    S: One,
    S: Sub<S, Output = S>,
    S: Copy,
    T: Add<T, Output = T>,
    T: Mul<S, Output = T>,
{
    let one: S = num::one();
    a * (one - t) + b * t
}

#[inline]
pub const fn is_power_of_2(v: i32) -> bool {
    (v != 0) && (v & (v - 1)) == 0
}

#[inline]
pub const fn round_up_pow_2(v: i32) -> i32 {
    let mut v = v;
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

pub trait Clampable {
    fn clamp(self, min: f32, max: f32) -> Self;
}

impl Clampable for f32 {
    fn clamp(self, min: f32, max: f32) -> f32 {
        clamp(self, min, max)
    }
}

pub fn find_interval<P>(size: usize, pred: P) -> usize
where
    P: Fn(usize) -> bool,
{
    let mut first = 0;
    let mut len = size;

    while len > 0 {
        let half = len >> 1;
        let middle = first + half;

        if pred(middle as usize) {
            first = middle + 1;
            len -= half + 1;
        } else {
            len = half;
        }
    }

    clamp(first as isize - 1, 0, size as isize - 2) as usize
}

pub fn gamma(n: u32) -> f32 {
    (n as f32 * MACHINE_EPSILON) / (1.0 - n as f32 * MACHINE_EPSILON)
}

/// Create an orthogonal coordinate system from a single vector.
pub fn coordinate_system(v1: &Vector3f) -> (Vector3f, Vector3f) {
    let v2 = if v1.x.abs() > v1.y.abs() {
        Vector3f::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
    } else {
        Vector3f::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt()
    };

    let v3 = v1.cross(&v2);

    (v2, v3)
}

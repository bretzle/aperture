mod bounds;
mod matrix;
mod normal;
mod point;
mod ray;
mod vector;

use num_traits::{Float, Num, NumAssignOps, One};
use std::{
    f32::consts::PI,
    ops::{Add, Mul, Sub},
};

pub use bounds::*;
pub use matrix::*;
pub use normal::*;
pub use point::*;
pub use ray::*;
pub use vector::*;

pub trait Number: Num + NumAssignOps + Copy + PartialOrd {}

pub fn lerp<S, T>(t: S, a: T, b: T) -> T
where
    S: One + Copy + Sub<S, Output = S>,
    T: Float + Add<T, Output = T> + Mul<S, Output = T>,
{
    let one: S = One::one();
    a * (one - t) + b * t
}

pub struct Angle(f32);

impl Angle {
    pub fn deg(val: f32) -> Self {
        Self(val)
    }

    pub fn rad(val: f32) -> Self {
        Self(val * 180.0 / PI)
    }

    pub fn as_deg(&self) -> f32 {
        self.0
    }

    pub fn as_rad(&self) -> f32 {
        self.0 * PI / 180.0
    }

	pub fn radians(deg: f32) -> f32 {
		deg * PI / 180.0
	}
}

pub fn clamp<T>(val: T, low: T, high: T) -> T
where
    T: PartialOrd,
{
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

/// When tracing spawned rays leaving the intersection point p, we
/// offset their origins enough to ensure that they are past the
/// boundary of the error box and thus won't incorrectly re-intersect
/// the surface.
pub fn pnt3_offset_ray_origin(
    p: Point3<f32>,
    p_error: &Vector3<f32>,
    n: &Normal3<f32>,
    w: &Vector3<f32>,
) -> Point3<f32> {
    let d = n.abs().dot_vec(p_error);
    let mut offset = Vector3::from(*n) * d;
    if w.dot_nrm(n) < 0.0 {
        offset = -offset;
    }
    let mut po = p + offset;
    // round offset point _po_ away from _p_
    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = next_float_up(po[i]);
        } else if offset[i] < 0.0 {
            po[i] = next_float_down(po[i]);
        }
    }
    po
}

/// Bump a floating-point value up to the next greater representable
/// floating-point value.
pub fn next_float_up(v: f32) -> f32 {
    if v.is_infinite() && v > 0.0 {
        v
    } else {
        let new_v = if v == -0.0 { 0.0 } else { v };
        let mut ui = new_v.to_bits();
        if new_v >= 0.0 {
            ui += 1;
        } else {
            ui -= 1;
        }
        f32::from_bits(ui)
    }
}

/// Bump a floating-point value down to the next smaller representable
/// floating-point value.
pub fn next_float_down(v: f32) -> f32 {
    if v.is_infinite() && v < 0.0 {
        v
    } else {
        let new_v = if v == 0.0 { -0.0 } else { v };
        let mut ui = new_v.to_bits();
        if new_v > 0.0 {
            ui -= 1;
        } else {
            ui += 1;
        }
        f32::from_bits(ui)
    }
}

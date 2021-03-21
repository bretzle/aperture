use log::debug;
use num::One;
use std::{
    f32::consts::TAU,
    ops::{Add, Mul, Sub},
};

use crate::geometry::{Point3f, Vector3f};

pub const MACHINE_EPSILON: f32 = f32::EPSILON * 0.5;

/// Linear interpolation between 2 values.
///
/// This version should be generic enough to linearly interpolate between 2 Spectrums using an f32
/// parameter.
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

/// Version of min() that works on `PartialOrd`, so it works for both u32 and f32.
pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a.lt(&b) {
        a
    } else {
        b
    }
}

/// Version of max() that works on `PartialOrd`, so it works for both u32 and f32.
pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a.gt(&b) {
        a
    } else {
        b
    }
}

pub fn gamma(n: u32) -> f32 {
    (n as f32 * MACHINE_EPSILON) / (1.0 - n as f32 * MACHINE_EPSILON)
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

// TODO
pub fn resolve_filename(filename: &str) -> String {
    debug!("Resolving filename {}", filename);
    filename.to_owned()
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

pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector3f {
    Vector3f::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn spherical_direction_vecs(
    sin_theta: f32,
    cos_theta: f32,
    phi: f32,
    x: &Vector3f,
    y: &Vector3f,
    z: &Vector3f,
) -> Vector3f {
    *x * (sin_theta * phi.cos()) + *y * (sin_theta * phi.sin()) + *z * cos_theta
}

pub fn SphericalTheta(v: &Vector3f) -> f32 {
    clamp(v.z, -1.0, 1.0).acos()
}

pub fn SphericalPhi(v: &Vector3f) -> f32 {
    let p = v.y.atan2(v.x);
    if p < 0.0 {
        p + TAU
    } else {
        p
    }
}

use crate::{
    geometry::{Bounds2i, Point2i, Point3f, Vector3f},
    spectrum::Spectrum,
};
use anyhow::Result;
use log::debug;
use num::One;
use std::{
    f32::consts::TAU,
    num::Wrapping,
    ops::{Add, Mul, Sub},
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
};

pub const MACHINE_EPSILON: f32 = f32::EPSILON * 0.5;
pub const INV_2_PI: f32 = 0.15915494309189533577;

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

const PCG32_DEFAULT_STATE: Wrapping<u64> = Wrapping(0x853c49e6748fea9b);
const PCG32_DEFAULT_STREAM: Wrapping<u64> = Wrapping(0xda3e39cb94b95bdb);
const PCG32_MULT: Wrapping<u64> = Wrapping(0x5851f42d4c957f2d);
pub const ONE_MINUS_EPSILON: f32 = 0.99999994f32;

#[derive(Copy, Clone)]
pub struct Rng {
    state: Wrapping<u64>,
    inc: Wrapping<u64>,
}

impl Rng {
    pub fn new() -> Self {
        Rng {
            state: PCG32_DEFAULT_STATE,
            inc: PCG32_DEFAULT_STREAM,
        }
    }

    pub fn uniform_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate * PCG32_MULT + self.inc;
        let xorshifted = Wrapping((((oldstate >> 18) ^ oldstate) >> 27).0 as u32);
        let rot = (oldstate >> 59).0 as u32;

        (xorshifted.0 >> rot) | (xorshifted.0 << ((!Wrapping(rot) + Wrapping(1)).0 & 31))
    }

    pub fn uniform_u32_bounded(&mut self, b: u32) -> u32 {
        let threshold = (!b + 1) & b;
        loop {
            let r = self.uniform_u32();
            if r >= threshold {
                return r % b;
            }
        }
    }

    pub fn uniform_f32(&mut self) -> f32 {
        (self.uniform_u32() as f32 * 2.3283064365386963E-10).min(ONE_MINUS_EPSILON)
    }

    pub fn set_sequence(&mut self, seed: u64) {
        self.state = Wrapping(0);
        self.inc = Wrapping((seed << 1) | 1);
        let _ = self.uniform_u32();
        self.state += PCG32_DEFAULT_STATE;
        let _ = self.uniform_u32();
    }
}

impl Default for Rng {
    fn default() -> Rng {
        Rng::new()
    }
}

#[derive(Default)]
pub struct AtomicFloat {
    bits: AtomicU32,
}

impl AtomicFloat {
    pub fn as_float(&self) -> f32 {
        f32::from_bits(self.bits.load(Ordering::Relaxed))
    }

    pub fn add(&mut self, v: f32) {
        let f = self.as_float() + v;
        let bits = f.to_bits();
        self.bits.store(bits, Ordering::Relaxed)
    }
}

pub fn write_image<P: AsRef<Path>>(
    _name: P,
    _rgb: &[f32],
    _output_bounds: &Bounds2i,
    _total_resolution: Point2i,
) -> Result<()> {
    todo!()
}

pub trait Clampable {
    fn clamp(self, min: f32, max: f32) -> Self;
}

impl Clampable for f32 {
    fn clamp(self, min: f32, max: f32) -> f32 {
        clamp(self, min, max)
    }
}

impl Clampable for Spectrum {
    fn clamp(self, min: f32, max: f32) -> Spectrum {
        Spectrum::rgb(
            Clampable::clamp(self.r, min, max),
            Clampable::clamp(self.g, min, max),
            Clampable::clamp(self.b, min, max),
        )
    }
}

pub fn has_extension<P: AsRef<Path>>(filename: P, extension: &str) -> bool {
    filename
        .as_ref()
        .extension()
        .map(|e| e == extension)
        .unwrap_or(false)
}

pub fn read_image<P: AsRef<Path>>(path: P) -> Result<(Vec<Spectrum>, Point2i)> {
    todo!()
}

#[inline]
pub fn is_power_of_2(v: i32) -> bool {
    (v != 0) && (v & (v - 1)) == 0
}

#[inline]
pub fn round_up_pow_2(v: i32) -> i32 {
    let mut v = v;
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

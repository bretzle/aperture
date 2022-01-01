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

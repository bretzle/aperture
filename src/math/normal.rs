use num_traits::{Float, Num, Signed};
use std::ops::Div;

use super::Vector3;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Normal3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn normalize(&self) -> Self
    where
        T: Float,
    {
        *self / self.length()
    }

    #[must_use]
    pub fn abs(&self) -> Self
    where
        T: Signed,
    {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Product of the Euclidean magnitudes of a normal (and a vector) and
    /// the cosine of the angle between them. A return value of zero means
    /// both are orthogonal, a value if one means they are codirectional.
    pub fn dot_vec(&self, v: &Vector3<T>) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl<T: Num + Copy> Div<T> for Normal3<T> {
    type Output = Normal3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Normal3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

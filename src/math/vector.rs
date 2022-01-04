use num_traits::{Float, Num, Signed};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use super::Normal3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.length_squared().sqrt()
    }

    pub fn has_nans(&self) -> bool
    where
        T: Float,
    {
        if cfg!(debug_assertions) {
            self.x.is_nan() || self.y.is_nan()
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn has_nans(&self) -> bool
    where
        T: Float,
    {
        if cfg!(debug_assertions) {
            self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
        } else {
            false
        }
    }

    #[must_use]
    pub fn abs(self) -> Self
    where
        T: Signed,
    {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> T
    where
        T: Float,
    {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn normalize(self) -> Self
    where
        T: Float,
    {
        self / self.length()
    }

    #[must_use]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn dot(self, v2: Self) -> T {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn dot_nrm(&self, n: &Normal3<T>) -> T {
        self.x * n.x + self.y * n.y + self.z * n.z
    }
}

impl<T> From<Normal3<T>> for Vector3<T> {
    fn from(n: Normal3<T>) -> Self {
        Self {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!(),
        }
    }
}

impl<T: Num + Neg<Output = T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Num + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Num> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Num> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

use super::{Vector2, Vector3};
use num_traits::{Float, Num};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

impl<T: Num> Sub for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec(vec: Vector3<T>) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
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
}

impl<T: Num> Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Num + Copy> Mul<T> for Point3<T> {
    type Output = Point3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Num> Add for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Num> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

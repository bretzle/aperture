use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use super::Vector;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn broadcast(val: f32) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
    }

    pub fn length_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_sqr())
    }

    pub fn normalize(&self) -> Normal {
        let len = self.length();
        Normal {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn face_forward(&self, v: &Vector) -> Normal {
        if super::dot(self, v) < 0f32 {
            -*self
        } else {
            *self
        }
    }
}

impl Add for Normal {
    type Output = Normal;

    fn add(self, rhs: Normal) -> Normal {
        Normal {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Normal {
    type Output = Normal;

    fn sub(self, rhs: Normal) -> Normal {
        Normal {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Normal {
    type Output = Normal;

    fn mul(self, rhs: Normal) -> Normal {
        Normal {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Normal {
    type Output = Normal;

    fn mul(self, rhs: f32) -> Normal {
        Normal {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Normal> for f32 {
    type Output = Normal;

    fn mul(self, rhs: Normal) -> Normal {
        Normal {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Normal {
    type Output = Normal;

    fn div(self, rhs: Normal) -> Normal {
        Normal {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Normal {
    type Output = Normal;

    fn div(self, rhs: f32) -> Normal {
        Normal {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Normal {
    type Output = Normal;

    fn neg(self) -> Normal {
        Normal {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Normal {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index into normal"),
        }
    }
}

impl IndexMut<usize> for Normal {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index into normal"),
        }
    }
}

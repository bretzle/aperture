use std::f32;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::linalg::{self, Vector};

/// Normal is a standard 3 component normal but transforms as a normal
/// normal when transformations are applied
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
    /// Initialize the normal and set values for x, y, z
    pub fn new(x: f32, y: f32, z: f32) -> Normal {
        Normal { x, y, z }
    }

    /// Initialize the normal with the same value of x, y, z
    pub fn broadcast(x: f32) -> Normal {
        Normal { x, y: x, z: x }
    }

    /// Compute the squared length of the normal
    pub fn length_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Compute the length of the normal
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_sqr())
    }

    /// Get a normalized copy of this normal
    #[must_use]
    pub fn normalized(&self) -> Normal {
        let len = self.length();
        Normal {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    /// Return a normal facing along the same direction as v
    #[must_use]
    pub fn face_forward(&self, v: &Vector) -> Normal {
        if linalg::dot(self, v) < 0f32 {
            -*self
        } else {
            *self
        }
    }
}

impl Add for Normal {
    type Output = Normal;
    /// Add two normals together
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
    /// Subtract two normals
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
    /// Multiply two normals
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
    /// Scale the normal by some value
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
    /// Scale the normal by some value
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
    /// Divide the normals components by the right hand side's components
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
    /// Divide the normals components by scalar
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
    /// Negate the normal
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
    /// Access the normal by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2 = z
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
    /// Access the normal by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2 = z
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index into normal"),
        }
    }
}

#[test]
fn test_face_fwd() {
    let n = Normal::new(1f32, 0f32, 0f32);
    let v = Vector::new(-1f32, 0f32, 0f32);
    let n_fwd = n.face_forward(&v);
    assert!(n_fwd == Normal::new(-1f32, 0f32, 0f32));
}

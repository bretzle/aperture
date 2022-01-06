use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    math::{clamp, Matrix, Vector3},
    transform::Transform,
};

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub v: Vector3<f32>,
    pub w: f32,
}

impl Quaternion {
    pub fn new(t: Transform) -> Self {
        let m = t.mat;
        let trace = m.m[0][0] + m.m[1][1] + m.m[2][2];

        if trace > 0.0 {
            let mut s = (trace + 1.0).sqrt();
            let w = s / 2.0;
            s = 0.5 / s;
            Self {
                v: Vector3 {
                    x: (m.m[2][1] - m.m[1][2]) * s,
                    y: (m.m[0][2] - m.m[2][0]) * s,
                    z: (m.m[1][0] - m.m[0][1]) * s,
                },
                w,
            }
        } else {
            let nxt: [usize; 3] = [1, 2, 0];
            let mut q: [f32; 3] = [0.0; 3];
            let mut i = if m.m[1][1] > m.m[0][0] { 1 } else { 0 };
            if m.m[2][2] > m.m[i][i] {
                i = 2;
            }
            let j = nxt[i];
            let k = nxt[j];
            let mut s = ((m.m[i][i] - (m.m[j][j] + m.m[k][k])) + 1.0).sqrt();
            q[i] = s * 0.5;
            if s != 0.0 {
                s = 0.5 / s;
            }
            let w = (m.m[k][j] - m.m[j][k]) * s;
            q[j] = (m.m[j][i] + m.m[i][j]) * s;
            q[k] = (m.m[k][i] + m.m[i][k]) * s;
            Self {
                v: Vector3 {
                    x: q[0],
                    y: q[1],
                    z: q[2],
                },
                w,
            }
        }
    }

    pub fn to_transform(self) -> Transform {
        let xx = self.v.x * self.v.x;
        let yy = self.v.y * self.v.y;
        let zz = self.v.z * self.v.z;
        let xy = self.v.x * self.v.y;
        let xz = self.v.x * self.v.z;
        let yz = self.v.y * self.v.z;
        let wx = self.v.x * self.w;
        let wy = self.v.y * self.w;
        let wz = self.v.z * self.w;

        let mut m = Matrix::default();
        m.m[0][0] = 1.0 - 2.0 * (yy + zz);
        m.m[0][1] = 2.0 * (xy + wz);
        m.m[0][2] = 2.0 * (xz - wy);
        m.m[1][0] = 2.0 * (xy - wz);
        m.m[1][1] = 1.0 - 2.0 * (xx + zz);
        m.m[1][2] = 2.0 * (yz + wx);
        m.m[2][0] = 2.0 * (xz + wy);
        m.m[2][1] = 2.0 * (yz - wx);
        m.m[2][2] = 1.0 - 2.0 * (xx + yy);

        // transpose since we are left-handed
        Transform {
            mat: m.transpose(),
            inv: m,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.v.dot(other.v) + self.w * other.w
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        *self / self.dot(self).sqrt()
    }

	/// Spherical linear interpolation
    pub fn slerp(t: f32, q1: &Self, q2: &Self) -> Self {
        let cos_theta = q1.dot(q2);
        if cos_theta > 0.9995 {
            (*q1 * (1.0 - t) + *q2 * t).normalize()
        } else {
            let theta = clamp(cos_theta, -1.0, 1.0).acos();
            let thetap = theta * t;
            let qperp = (*q2 - *q1 * cos_theta).normalize();
            *q1 * thetap.cos() + qperp * thetap.sin()
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            v: Vector3::default(),
            w: 1.0,
        }
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion {
            v: self.v + rhs.v,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion {
            v: self.v - rhs.v,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion {
            v: self.v * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        Quaternion {
            v: self.v / rhs,
            w: self.w / rhs,
        }
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Self::Output {
        Quaternion {
            v: -self.v,
            w: -self.w,
        }
    }
}

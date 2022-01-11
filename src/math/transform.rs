use super::{cross, Matrix, Point, Vector};
use crate::matrix;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub mat: Matrix,
    pub inv: Matrix,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        mat: Matrix::IDENTITY,
        inv: Matrix::IDENTITY,
    };

    pub fn from_matrix(mat: Matrix) -> Self {
        Self {
            mat,
            inv: mat.inverse(),
        }
    }

    pub fn from_pair(mat: Matrix, inv: Matrix) -> Self {
        Self { mat, inv }
    }

    pub fn translate(v: &Vector) -> Self {
        Self {
            mat: matrix! {
                1.0, 0.0, 0.0, v.x;
                0.0, 1.0, 0.0, v.y;
                0.0, 0.0, 1.0, v.z;
                0.0, 0.0, 0.0, 1.0;
            },
            inv: matrix! {
                1.0, 0.0, 0.0, -v.x;
                0.0, 1.0, 0.0, -v.y;
                0.0, 0.0, 1.0, -v.z;
                0.0, 0.0, 0.0, 1.0;
            },
        }
    }

    pub fn scale(v: &Vector) -> Self {
        Self {
            mat: matrix! {
                v.x, 0.0, 0.0, 0.0;
                0.0, v.y, 0.0, 0.0;
                0.0, 0.0, v.z, 0.0;
                0.0, 0.0, 0.0, 1.0;
            },
            inv: matrix! {
                1.0 / v.x, 0.0, 0.0, 0.0;
                0.0, 1.0 / v.y, 0.0, 0.0;
                0.0, 0.0, 1.0 / v.z, 0.0;
                0.0, 0.0, 0.0, 1.0;
            },
        }
    }

    pub fn rotate_x(deg: f32) -> Self {
        let r = deg.to_radians();
        let s = r.sin();
        let c = r.cos();
        let mat = matrix! {
            1.0, 0.0, 0.0, 0.0;
            0.0, c, -s, 0.0;
            0.0, s, c, 0.0;
            0.0, 0.0, 0.0, 1.0;
        };

        Self {
            mat,
            inv: mat.transpose(),
        }
    }

    pub fn rotate_y(deg: f32) -> Self {
        let r = deg.to_radians();
        let s = r.sin();
        let c = r.cos();
        let mat = matrix! {
            c, 0.0, s, 0.0;
            0.0, 1.0, 0.0, 0.0;
            -s, 0.0, c, 0.0;
            0.0, 0.0, 0.0, 1.0;
        };

        Self {
            mat,
            inv: mat.transpose(),
        }
    }

    pub fn rotate_z(deg: f32) -> Self {
        let r = deg.to_radians();
        let s = r.sin();
        let c = r.cos();
        let mat = matrix! {
            c, -s, 0.0, 0.0;
            s, c, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0;
            0.0, 0.0, 0.0, 1.0;
        };

        Self {
            mat,
            inv: mat.transpose(),
        }
    }

    pub fn rotate(axis: Vector, deg: f32) -> Self {
        let a = axis.normalize();
        let r = deg.to_radians();
        let s = r.sin();
        let c = r.cos();
        let mut mat = Matrix::IDENTITY;

        *mat.at_mut(0, 0) = a.x * a.x + (1.0 - a.x * a.x) * c;
        *mat.at_mut(0, 1) = a.x * a.y * (1.0 - c) - a.z * s;
        *mat.at_mut(0, 2) = a.x * a.z * (1.0 - c) + a.y * s;

        *mat.at_mut(1, 0) = a.x * a.y * (1.0 - c) + a.z * s;
        *mat.at_mut(1, 1) = a.y * a.y + (1.0 - a.y * a.y) * c;
        *mat.at_mut(1, 2) = a.y * a.z * (1.0 - c) - a.x * s;

        *mat.at_mut(2, 0) = a.x * a.z * (1.0 - c) - a.y * s;
        *mat.at_mut(2, 1) = a.y * a.z * (1.0 - c) + a.x * s;
        *mat.at_mut(2, 2) = a.z * a.z + (1.0 - a.z * a.z) * c;

        Self {
            mat,
            inv: mat.transpose(),
        }
    }

    pub fn look_at(pos: &Point, center: &Point, up: &Vector) -> Self {
        let dir = (*center - *pos).normalize();
        let left = cross(up, &dir).normalize();
        let u = cross(&dir, &left).normalize();
        let mut mat = Matrix::IDENTITY;

        for i in 0..3 {
            *mat.at_mut(i, 0) = -left[i];
            *mat.at_mut(i, 1) = u[i];
            *mat.at_mut(i, 2) = dir[i];
            *mat.at_mut(i, 3) = pos[i];
        }

        Self {
            mat,
            inv: mat.inverse(),
        }
    }

    pub fn perspective(fovy: f32, near: f32, far: f32) -> Self {
        let proj_div = matrix! {
            1.0, 0.0, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, far / (far - near), -far * near / (far - near);
            0.0, 0.0, 1.0, 0.0;
        };
        let inv_tan = 1.0 / (fovy.to_radians() / 2.0).tan();

        Transform::scale(&Vector::new(inv_tan, inv_tan, 1.0)) * Transform::from_matrix(proj_div)
    }

    pub fn inverse(&self) -> Self {
        Self {
            mat: self.inv,
            inv: self.mat,
        }
    }

    pub fn has_scale(&self) -> bool {
        todo!()
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        Transform {
            mat: self.mat * rhs.mat,
            inv: rhs.inv * self.inv,
        }
    }
}

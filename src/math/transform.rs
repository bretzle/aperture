use super::{cross, Matrix, Normal, Point, Ray, Vector};
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

    /// Multiply the point by the inverse transformation
    /// TODO: These inverse mults are a bit hacky since Rust doesn't currently
    /// have function overloading, clean up when it's added
    pub fn inv_mul_point(&self, p: &Point) -> Point {
        let mut res = Point::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.inv.at(i, 0) * p.x
                + *self.inv.at(i, 1) * p.y
                + *self.inv.at(i, 2) * p.z
                + *self.inv.at(i, 3);
        }
        let w = *self.inv.at(3, 0) * p.x
            + *self.inv.at(3, 1) * p.y
            + *self.inv.at(3, 2) * p.z
            + *self.inv.at(3, 3);
        if (w - 1.0).abs() < f32::EPSILON {
            res / w
        } else {
            res
        }
    }
    /// Multiply the vector with the inverse transformation
    pub fn inv_mul_vector(&self, v: &Vector) -> Vector {
        let mut res = Vector::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.inv.at(i, 0) * v.x + *self.inv.at(i, 1) * v.y + *self.inv.at(i, 2) * v.z;
        }
        res
    }
    /// Multiply the normal with the inverse transformation
    pub fn inv_mul_normal(&self, n: &Normal) -> Normal {
        let mut res = Normal::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.mat.at(0, i) * n.x + *self.mat.at(1, i) * n.y + *self.mat.at(2, i) * n.z;
        }
        res
    }
    /// Multiply the ray with the inverse transformation
    pub fn inv_mul_ray(&self, ray: &Ray) -> Ray {
        let mut res = *ray;
        res.o = self.inv_mul_point(&res.o);
        res.d = self.inv_mul_vector(&res.d);
        res
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

impl Mul<Point> for Transform {
    type Output = Point;
    /// Multiply the point by the transform to apply the transformation
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, p: Point) -> Point {
        let mut res = Point::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.mat.at(i, 0) * p.x
                + *self.mat.at(i, 1) * p.y
                + *self.mat.at(i, 2) * p.z
                + *self.mat.at(i, 3);
        }
        let w = *self.mat.at(3, 0) * p.x
            + *self.mat.at(3, 1) * p.y
            + *self.mat.at(3, 2) * p.z
            + *self.mat.at(3, 3);
        if (w - 1.0).abs() < f32::EPSILON {
            res / w
        } else {
            res
        }
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;
    /// Multiply the vector by the transform to apply the transformation
    fn mul(self, v: Vector) -> Vector {
        let mut res = Vector::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.mat.at(i, 0) * v.x + *self.mat.at(i, 1) * v.y + *self.mat.at(i, 2) * v.z;
        }
        res
    }
}

impl Mul<Normal> for Transform {
    type Output = Normal;
    /// Multiply the normal by the transform to apply the transformation
    fn mul(self, n: Normal) -> Normal {
        let mut res = Normal::broadcast(0.0);
        for i in 0..3 {
            res[i] = *self.inv.at(0, i) * n.x + *self.inv.at(1, i) * n.y + *self.inv.at(2, i) * n.z;
        }
        res
    }
}

impl Mul<Ray> for Transform {
    type Output = Ray;
    /// Multiply the ray by the transform to apply the transformation
    fn mul(self, ray: Ray) -> Ray {
        let mut res = ray;
        res.o = self * res.o;
        res.d = self * res.d;
        res
    }
}

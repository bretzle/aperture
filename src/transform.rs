use crate::math::{Matrix, Point3, Vector3};

#[derive(Debug, Default, Clone, Copy)]
pub struct Transform {
    pub mat: Matrix,
    pub inv: Matrix,
}

impl Transform {
    pub fn translate(delta: Vector3<f32>) -> Self {
        Self {
            mat: Matrix::new(
                [1.0, 0.0, 0.0, delta.x],
                [0.0, 1.0, 0.0, delta.y],
                [0.0, 0.0, 1.0, delta.z],
                [0.0, 0.0, 0.0, 1.0],
            ),
            inv: Matrix::new(
                [1.0, 0.0, 0.0, -delta.x],
                [0.0, 1.0, 0.0, -delta.y],
                [0.0, 0.0, 1.0, -delta.z],
                [0.0, 0.0, 0.0, 1.0],
            ),
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            mat: Matrix::new(
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ),
            inv: Matrix::new(
                [1.0 / x, 0.0, 0.0, 0.0],
                [0.0, 1.0 / y, 0.0, 0.0],
                [0.0, 0.0, 1.0 / z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ),
        }
    }

    pub fn inverse(&self) -> Transform {
        Self {
            mat: self.inv,
            inv: self.mat,
        }
    }

    pub fn is_identity(&self) -> bool {
        self.mat == Matrix::IDENTITY
    }

    pub fn transform_point(&self, p: Point3<f32>) -> Point3<f32> {
        let Point3 { x, y, z } = p;
        let mat = &self.mat.m;

        let xp = mat[0][0] * x + mat[0][1] * y + mat[0][2] * z + mat[0][3];
        let yp = mat[1][0] * x + mat[1][1] * y + mat[1][2] * z + mat[1][3];
        let zp = mat[2][0] * x + mat[2][1] * y + mat[2][2] * z + mat[2][3];
        let wp = mat[3][0] * x + mat[3][1] * y + mat[3][2] * z + mat[3][3];

        assert!(wp != 0.0, "wp = {:?} != 0.0", wp);

        if wp == 1.0 {
            Point3 {
                x: xp,
                y: yp,
                z: zp,
            }
        } else {
            Point3 {
                x: xp / wp,
                y: yp / wp,
                z: zp / wp,
            }
        }
    }

    pub fn to_matrix(self) -> Matrix {
        self.mat
    }
}

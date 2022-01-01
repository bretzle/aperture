use crate::{
    math::{Matrix, Point3, Vector3},
    quaternion::Quaternion,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

    #[must_use]
    pub fn inverse(&self) -> Self {
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

#[derive(Debug, Default, Copy, Clone)]
pub struct DerivativeTerm {
    kc: f32,
    kx: f32,
    ky: f32,
    kz: f32,
}

impl DerivativeTerm {
    pub fn eval(&self, p: &Point3<f32>) -> f32 {
        self.kc + self.kx * p.x + self.ky * p.y + self.kz * p.z
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AnimatedTransform {
    start_transform: Transform,
    end_transform: Transform,
    start_time: f32,
    end_time: f32,
    actually_animated: bool,
    t: [Vector3<f32>; 2],
    r: [Quaternion; 2],
    s: [Matrix; 2],
    has_rotation: bool,
    c1: [DerivativeTerm; 3],
    c2: [DerivativeTerm; 3],
    c3: [DerivativeTerm; 3],
    c4: [DerivativeTerm; 3],
    c5: [DerivativeTerm; 3],
}

impl AnimatedTransform {
    pub fn new(
        start_transform: Transform,
        end_transform: Transform,
        start_time: f32,
        end_time: f32,
    ) -> Self {
        let mut at = Self {
            actually_animated: start_transform != end_transform,
            start_transform,
            end_transform,
            start_time,
            end_time,
            ..Default::default()
        };

        AnimatedTransform::decompose(
            &start_transform.mat,
            &mut at.t[0],
            &mut at.r[0],
            &mut at.s[0],
        );
        AnimatedTransform::decompose(&end_transform.mat, &mut at.t[1], &mut at.r[1], &mut at.s[1]);
        // flip _r[1]_ if needed to select shortest path
        if at.r[0].dot(&at.r[1]) < 0.0 {
            at.r[1] = -at.r[1];
        }
        at.has_rotation = at.r[0].dot(&at.r[1]) < 0.9995;

        // compute terms of motion derivative function
        if at.has_rotation {
            todo!()
        }

        at
    }

    fn decompose(m: &Matrix, t: &mut Vector3<f32>, rquat: &mut Quaternion, s: &mut Matrix) {
        // extract translation from transformation matrix
        t.x = m.m[0][3];
        t.y = m.m[1][3];
        t.z = m.m[2][3];
        // compute new transformation matrix _m_ without translation
        let mut matrix = *m;
        for i in 0..3 {
            matrix.m[i][3] = 0.0;
            matrix.m[3][i] = 0.0;
        }
        matrix.m[3][3] = 1.0;
        // extract rotation _r_ from transformation matrix
        let mut norm;
        let mut r = matrix;
        for _ in 0..100 {
            // compute next matrix _rnext_ in series
            let rit = r.transpose().inverse().unwrap();
            let rnext = Matrix::with(|i, j| 0.5 * (r.m[i][j] + rit.m[i][j]));

            // compute norm of difference between _r_ and _rnext_
            norm = 0.0f32;
            for i in 0..3 {
                let n = (r.m[i][0] - rnext.m[i][0]).abs()
                    + (r.m[i][1] - rnext.m[i][1]).abs()
                    + (r.m[i][2] - rnext.m[i][2]).abs();
                norm = norm.max(n);
            }
            r = rnext;

            if norm <= 0.0001 {
                break;
            }
        }

        let transform = Transform {
            mat: r,
            inv: r.inverse().unwrap(),
        };

        *rquat = Quaternion::new(transform);

        // compute scale _S_ using rotation and original matrix
        *s = r.inverse().unwrap() * *m;
    }
}

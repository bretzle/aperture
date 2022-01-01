use super::{Angle, Vector3};
use crate::transform::Transform;
use std::ops::{Index, IndexMut, Mul};

#[macro_export]
macro_rules! matrix {
	( $( $( $val:expr ),+ );* ; ) => {
		Matrix::new( $( [$( $val as f32 ),+] ),* )
	};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

impl Matrix {
    pub const IDENTITY: Self = matrix! {
        1., 0., 0., 0.;
        0., 1., 0., 0.;
        0., 0., 1., 0.;
        0., 0., 0., 1.;
    };

    pub const fn new(r0: [f32; 4], r1: [f32; 4], r2: [f32; 4], r3: [f32; 4]) -> Self {
        Self {
            m: [r0, r1, r2, r3],
        }
    }

    pub fn with<F: Fn(usize, usize) -> f32>(f: F) -> Self {
        let mut mat = Matrix::IDENTITY;

        for i in 0..4 {
            for j in 0..4 {
                mat[i][j] = f(i, j);
            }
        }

        mat
    }

    #[must_use]
    pub const fn transpose(&self) -> Self {
        Self {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ],
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut indxc = vec![0; 4];
        let mut indxr = vec![0; 4];
        let mut ipiv = vec![0; 4];

        let mut minv = *self;

        for i in 0..4 {
            let mut irow = 0;
            let mut icol = 0;
            let mut big = 0.0;
            // choose pivot
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for (k, item) in ipiv.iter().enumerate().take(4) {
                        if *item == 0 {
                            let abs = (minv.m[j][k]).abs();
                            if abs >= big {
                                big = abs;
                                irow = j;
                                icol = k;
                            }
                        } else if *item > 1 {
                            println!("Singular matrix in MatrixInvert");
                            return None;
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            // swap rows _irow_ and _icol_ for pivot
            if irow != icol {
                for k in 0..4 {
                    // C++: std::swap(minv[irow][k], minv[icol][k]);
                    let swap = minv.m[irow][k];
                    minv.m[irow][k] = minv.m[icol][k];
                    minv.m[icol][k] = swap;
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv.m[icol][icol] == 0.0 {
                println!("Singular matrix in MatrixInvert");
                return None;
            }
            // set $m[icol][icol]$ to one by scaling row _icol_ appropriately
            let pivinv = 1.0 / minv.m[icol][icol];
            minv.m[icol][icol] = 1.0;
            for j in 0..4 {
                minv.m[icol][j] *= pivinv;
            }
            // subtract this row from others to zero out their columns
            for j in 0..4 {
                if j != icol {
                    let save = minv.m[j][icol];
                    minv.m[j][icol] = 0.0;
                    for k in 0..4 {
                        minv.m[j][k] -= minv.m[icol][k] * save;
                    }
                }
            }
        }

        // swap columns to reflect permutation
        for i in 0..4 {
            let j = 3 - i;
            if indxr[j] != indxc[j] {
                for k in 0..4 {
                    minv.m[k].swap(indxr[j], indxc[j])
                }
            }
        }

        Some(minv)
    }

    pub fn from_diagonal(diag: Vector3<f32>) -> Self {
        matrix! {
            diag.z, 0., 0., 0.;
            0., diag.y, 0., 0.;
            0., 0., diag.z, 0.;
            0., 0., 0., 1.0;
        }
    }

    pub fn to_transform(self) -> Transform {
        Transform {
            mat: self,
            inv: self.inverse().unwrap(),
        }
    }

    pub fn from_axis_angle(_axis: Vector3<f32>, _angle: Angle) -> Matrix {
        todo!()
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::IDENTITY
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut r = Self::IDENTITY;
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self[i][0] * rhs[0][j]
                    + self[i][1] * rhs[1][j]
                    + self[i][2] * rhs[2][j]
                    + self[i][3] * rhs[3][j];
            }
        }
        r
    }
}

impl Index<usize> for Matrix {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn approx(a: Matrix, b: Matrix) -> bool {
        let a = a.m;
        let b = b.m;
        !a.iter().enumerate().any(|(x, row)| {
            row.iter()
                .enumerate()
                .any(|(y, val)| !((b[x][y] - val).abs() < 0.1E-4))
        })
    }

    #[test]
    fn inverse() {
        let a = matrix![
            -5,  2,  6, -8;
             1, -5,  1,  8;
             7,  7, -6, -7;
             1, -3,  7,  4;
        ];
        assert!(approx(
            a.inverse().unwrap(),
            matrix![
                0.21805,  0.45113,  0.24060, -0.04511;
               -0.80827, -1.45677, -0.44361,  0.52068;
               -0.07895, -0.22368, -0.05263,  0.19737;
               -0.52256, -0.81391, -0.30075,  0.30639;
            ]
        ));

        let b = matrix![
             8, -5,  9,  2;
             7,  5,  6,  1;
            -6,  0,  9,  6;
            -3,  0, -9, -4;
        ];
        assert!(approx(
            b.inverse().unwrap(),
            matrix![
                -0.15385, -0.15385, -0.28205, -0.53846;
                -0.07692,  0.12308,  0.02564,  0.03077;
                 0.35897,  0.35897,  0.43590,  0.92308;
                -0.69231, -0.69231, -0.76923, -1.92308;
            ]
        ));

        let c = matrix![
             9,  3,  0,  9;
            -5, -2, -6, -3;
            -4,  9,  6,  4;
            -7,  6,  6,  2;
        ];
        assert!(approx(
            c.inverse().unwrap(),
            matrix![
                -0.04074, -0.07778,  0.14444, -0.22222;
                -0.07778,  0.03333,  0.36667, -0.33333;
                -0.02901, -0.14630, -0.10926,  0.12963;
                 0.17778,  0.06667, -0.26667,  0.33333;
            ]
        ));
    }

    #[test]
    fn multiply() {
        let a = matrix![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b = matrix![
            -2, 1, 2,  3;
             3, 2, 1, -1;
             4, 3, 6,  5;
             1, 2, 7,  8;
        ];
        let c = matrix![
            20, 22,  50,  48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26,  46,  42;
        ];

        assert_eq!(a * b, c);
    }
}

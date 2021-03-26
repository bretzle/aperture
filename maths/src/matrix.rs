use std::ops::{Index, IndexMut, Mul};

use log::error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub data: [[f32; 4]; 4],
}

impl Matrix {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&self) -> Self {
        Self::new([
            [self[0][0], self[1][0], self[2][0], self[3][0]],
            [self[0][1], self[1][1], self[2][1], self[3][1]],
            [self[0][2], self[1][2], self[2][2], self[3][2]],
            [self[0][3], self[1][3], self[2][3], self[3][3]],
        ])
    }

    #[allow(clippy::needless_range_loop)]
    pub fn inverse(&self) -> Self {
        let mut indxc = [0usize; 4];
        let mut indxr = [0usize; 4];
        let mut ipiv = [0usize; 4];
        let mut minv = self.data;

        for i in 0..4 {
            let mut irow = 0;
            let mut icol = 0;
            let mut big = 0.0;

            // Choose pivot
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for k in 0..4 {
                        if ipiv[k] == 0 {
                            if f32::abs(minv[j][k]) >= big {
                                big = f32::abs(minv[j][k]);
                                irow = j;
                                icol = k;
                            }
                        } else if ipiv[k] > 1 {
                            error!("Singular matrix in Matrix4x4::inverse()");
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            // Swap rows `irow` and `icol` for pivot
            if irow != icol {
                for k in 0..4 {
                    let tmp = minv[irow][k];
                    minv[irow][k] = minv[icol][k];
                    minv[icol][k] = tmp;
                    // This doesn't work because I can't borrow minv mutably twice :(
                    // ::std::mem::swap(&mut minv[irow][k], &mut minv[icol][k]);
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol][icol] == 0.0 {
                error!("Singular matrix in Matrix4x4::inverse()");
            }

            // Set `m[icol][icol]` to one by rscaling row `icol` appropriately
            let pivinv = 1.0 / minv[icol][icol];
            minv[icol][icol] = 1.0;
            for j in 0..4 {
                minv[icol][j] *= pivinv;
            }

            // Substract this row from others to zero out their columns
            for j in 0..4 {
                if j != icol {
                    let save = minv[j][icol];
                    minv[j][icol] = 0.0;
                    for k in 0..4 {
                        minv[j][k] -= minv[icol][k] * save;
                    }
                }
            }
        }

        // Swap columns to reflect permutation
        for j in (0..4).rev() {
            if indxr[j] != indxc[j] {
                for k in 0..4 {
                    minv[k].swap(indxr[j], indxc[j]);
                }
            }
        }

        Self { data: minv }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Self::Output {
        let mut r = Matrix::default();
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
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    fn approx(a: Matrix, b: Matrix) -> bool {
        !a.data.iter().enumerate().any(|(x, row)| {
            row.iter()
                .enumerate()
                .any(|(y, val)| !((b[x][y] - val).abs() < 0.1E-4))
        })
    }

    #[test]
    fn matrix_equality() {
        let a = matrix![
            1,2,3,4;
            5,6,7,8;
            9,8,7,6;
            5,4,3,2;
        ];
        let b = matrix![
            1,2,3,4;
            5,6,7,8;
            9,8,7,6;
            5,4,3,2;
        ];
        let c = matrix![
            2,3,4,5;
            6,7,8,9;
            8,7,6,5;
            4,3,2,1;
        ];

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn multiply_matrix_by_matrix() {
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

        assert_eq!(&a * &b, c);
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = matrix![
            0, 1, 2, 4;
            1, 2, 4, 8;
            2, 4, 8, 16;
            4, 8, 16, 32;
        ];
        let actual = &a * &Matrix::identity();

        assert_eq!(a, actual);
    }

    #[test]
    fn transpose_matrix() {
        let a = matrix![
            0, 9, 3, 0;
            9, 8, 0, 8;
            1, 8, 5, 3;
            0, 0, 5, 8;
        ];
        let expected = matrix![
            0, 9, 1, 0;
            9, 8, 8, 0;
            3, 0, 5, 5;
            0, 8, 3, 8;
        ];

        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn transpose_identity() {
        let a = Matrix::identity();
        let expected = a.clone();

        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn invert_matrix() {
        let a = matrix![
            -5,  2,  6, -8;
             1, -5,  1,  8;
             7,  7, -6, -7;
             1, -3,  7,  4;
        ];
        assert!(approx(
            a.inverse(),
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
            b.inverse(),
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
            c.inverse(),
            matrix![
                -0.04074, -0.07778,  0.14444, -0.22222;
                -0.07778,  0.03333,  0.36667, -0.33333;
                -0.02901, -0.14630, -0.10926,  0.12963;
                 0.17778,  0.06667, -0.26667,  0.33333;
            ]
        ));
    }

    // #[test]
    // fn inverse_multiplication() {
    // 	let a = matrix![ 4, 4 =>
    // 		 3., -9.,  7.,  3.;
    // 		 3., -8.,  2., -9.;
    // 		-4.,  4.,  4.,  1.;
    // 		-6.,  5., -1.,  1.;
    // 	];
    // 	let b = matrix![ 4, 4 =>
    // 		8.,  2., 2., 2.;
    // 		3., -1., 7., 0.;
    // 		7.,  0., 5., 4.;
    // 		6., -2., 0., 5.;
    // 	];

    // 	let c = &a * &b;
    // 	assert_approx_eq!(c * b.invert().unwrap(), &a);
    // }
}

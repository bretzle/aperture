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
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
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

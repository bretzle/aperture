use core::{
    fmt,
    ops::{Add, Index, IndexMut, Mul, Sub},
};
use log::error;

// region: const macros

macro_rules! const_create {
    (
		$arr:ident,
		$i:ident = $imin:expr => $imax:expr,
		$j:ident = $jmin:expr => $jmax:expr,
		$block:block
	) => {{
        let mut $arr = [[0.0; $imax]; $jmax];
        const_loop! {
            $i = $imin => $imax,
            $j = $jmin => $jmax,
            $block
        }
        $arr
    }};
}

macro_rules! const_loop {
	(
		$i:ident = $imin:expr => $imax:expr,
		$j:ident = $jmin:expr => $jmax:expr,
		$block:block
	) => {
		let mut $i = $imin;
        let mut $j = $jmin;
		loop {
            if $j == $jmax {
                break;
            }
			loop {
				if $i == $imax {
					break;
				}
				$block
				$i += 1;
			}
			$i = $imin;
			$j += 1;
        }
	};
}

// endregion

#[derive(Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub data: [[f32; R]; C],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const fn new(data: [[f32; R]; C]) -> Self {
        Self { data }
    }

    pub const fn zeroed() -> Self {
        Self::new([[0.0; R]; C])
    }

    pub const fn cols(&self) -> usize {
        C
    }

    pub const fn rows(&self) -> usize {
        R
    }

    pub const fn transpose(&self) -> Matrix<C, R> {
        let data = const_create! {
            data,
            i = 0 => C,
            j = 0 => R,
            { data[i][j] = self.data[j][i] }
        };

        Matrix { data }
    }
}

impl<const R: usize> Matrix<R, R> {
    pub const fn identity() -> Self {
        let data = {
            let mut data = [[0.0; R]; R];
            let mut i = 0;
            loop {
                if i == R {
                    break;
                }
                data[i][i] = 1.0;
                i += 1;
            }
            data
        };

        Self { data }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn inverse(&self) -> Self {
        let mut indxc = [0usize; 4];
        let mut indxr = [0usize; 4];
        let mut ipiv = [0usize; 4];
        let mut minv = self.data;

        for i in 0..R {
            let mut irow = 0;
            let mut icol = 0;
            let mut big = 0.0;

            // Choose pivot
            for j in 0..R {
                if ipiv[j] != 1 {
                    for k in 0..R {
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
                for k in 0..R {
                    let tmp = minv[irow][k];
                    minv[irow][k] = minv[icol][k];
                    minv[icol][k] = tmp;
                    // This doesn't work because I can't borrow minv mutably
                    // twice :( ::std::mem::swap(&mut
                    // minv[irow][k], &mut minv[icol][k]);
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
            for j in 0..R {
                minv[icol][j] *= pivinv;
            }

            // Substract this row from others to zero out their columns
            for j in 0..R {
                if j != icol {
                    let save = minv[j][icol];
                    minv[j][icol] = 0.0;
                    for k in 0..R {
                        minv[j][k] -= minv[icol][k] * save;
                    }
                }
            }
        }

        // Swap columns to reflect permutation
        for j in (0..R).rev() {
            if indxr[j] != indxc[j] {
                for k in 0..R {
                    minv[k].swap(indxr[j], indxc[j]);
                }
            }
        }

        Self { data: minv }
    }
}

// region: const operations

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub const fn add(&self, rhs: &Self) -> Self {
        let data = const_create! {
            data,
            i = 0 => R,
            j = 0 => C,
            { data[j][i] = self.data[j][i] + rhs.data[j][i] }
        };
        Self { data }
    }

    pub const fn sub(&self, rhs: &Self) -> Self {
        let data = const_create! {
            data,
            i = 0 => R,
            j = 0 => C,
            { data[j][i] = self.data[j][i] - rhs.data[j][i] }
        };
        Self { data }
    }

    pub const fn mul<const K: usize>(&self, rhs: &Matrix<C, K>) -> Matrix<R, K> {
        let data = const_create! {
            data,
            i = 0 => R,
            j = 0 => K,
            {
                data[i][j] = {
                    // Initialize sum with the first value so that we never have to initialize it
                    // with zero, which can be difficult with generic numeric types
                    let mut sum = self.data[i][0] * rhs.data[0][j];

                    // Add the remainder of the n-tuple
                    let mut k = 1;
                    loop {
                        if k == C {
                            break;
                        }
                        sum += self.data[i][k] * rhs.data[k][j];
                        k += 1;
                    }

                    // Return the sum
                    sum
                }
            }
        };

        Matrix { data }
    }
}

// endregion

// region: std::ops impls

impl<const R: usize, const C: usize> Add for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl<const R: usize, const C: usize> Sub for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<const R: usize, const C: usize, const K: usize> Mul<&Matrix<C, K>> for &Matrix<R, C> {
    type Output = Matrix<R, K>;

    fn mul(self, rhs: &Matrix<C, K>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f32; R];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

// endregion

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<const R: usize, const C: usize> fmt::Debug for Matrix<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", R, C)?;
        for y in 0..C {
            for x in 0..R {
                write!(f, "{} ", self[y][x])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    fn approx<const R: usize, const C: usize>(a: Matrix<R, C>, b: Matrix<R, C>) -> bool {
        !a.data.iter().enumerate().any(|(x, row)| {
            row.iter()
                .enumerate()
                .any(|(y, val)| !((b[x][y] - val).abs() < 0.1E-4))
        })
    }

    #[test]
    fn add() {
        let a = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let b = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let expected = matrix![
            2.0, 4.0, 6.0;
            8.0, 10.0, 12.0;
        ];

        assert_eq!(Matrix::add(&a, &b), expected);
    }

    #[test]
    fn sub() {
        let a = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let b = matrix![
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let expected = Matrix::zeroed();

        assert_eq!(Matrix::sub(&a, &b), expected);
    }

    #[test]
    fn mul() {
        let a = matrix![
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = matrix![
            20, 22,  50,  48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26,  46,  42;
        ];
        let d = matrix![
            36,   30, 24, 18;
            17,   22, 27, 32;
            98,   94, 90, 86;
            114, 102, 90, 78;
        ];

        assert_eq!(Matrix::mul(&a, &b), c);
        assert_eq!(Matrix::mul(&b, &a), d);
    }

    #[test]
    fn identity() {
        let a = Matrix::identity();
        let expected = matrix![
            1, 0, 0, 0;
            0, 1, 0, 0;
            0, 0, 1, 0;
            0, 0, 0, 1;
        ];

        assert_eq!(a, expected)
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
        let a = Matrix::<4, 4>::identity();
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
}

use std::{
    fmt,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

#[derive(Clone)]
pub struct CMatrix<const R: usize, const C: usize> {
    data: [[f32; R]; C],
}

impl<const R: usize, const C: usize> CMatrix<R, C> {
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

    pub const fn transpose(&self) -> CMatrix<C, R> {
        let data = {
            let mut data = [[0.0; C]; R];

            let mut i = 0;
            let mut j = 0;
            loop {
                if i == R {
                    break;
                }
                loop {
                    if j == C {
                        break;
                    }
                    data[i][j] = self.data[j][i];
                    j += 1;
                }
                i += 1;
            }

            data
        };

        CMatrix { data }
    }
}

impl<const R: usize> CMatrix<R, R> {
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
}

// region: const operations

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

impl<const R: usize, const C: usize> CMatrix<R, C> {
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

    pub const fn mul<const K: usize>(&self, rhs: &CMatrix<C, K>) -> CMatrix<R, K> {
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

        CMatrix { data }
    }
}

// endregion

// region: std::ops impls

impl<const R: usize, const C: usize> Add for &CMatrix<R, C> {
    type Output = CMatrix<R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl<const R: usize, const C: usize> Sub for &CMatrix<R, C> {
    type Output = CMatrix<R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<const R: usize, const C: usize, const K: usize> Mul<&CMatrix<C, K>> for &CMatrix<R, C> {
    type Output = CMatrix<R, K>;

    fn mul(self, rhs: &CMatrix<C, K>) -> Self::Output {
        self.mul(&rhs)
    }
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix;

    #[test]
    fn add() {
        let a = matrix![CC
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let b = matrix![CC
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let expected = matrix![CC
            2.0, 4.0, 6.0;
            8.0, 10.0, 12.0;
        ];

        assert_eq!(CMatrix::add(&a, &b), expected);
    }

    #[test]
    fn sub() {
        let a = matrix![CC
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let b = matrix![CC
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0;
        ];
        let expected = CMatrix::zeroed();

        assert_eq!(CMatrix::sub(&a, &b), expected);
    }

    #[test]
    fn mul() {
        let a = matrix![CC
            1, 2, 3, 4;
            5, 6, 7, 8;
            9, 8, 7, 6;
            5, 4, 3, 2;
        ];
        let b = CMatrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = matrix![CC
            20, 22,  50,  48;
            44, 54, 114, 108;
            40, 58, 110, 102;
            16, 26,  46,  42;
        ];
        let d = matrix![CC
            36,   30, 24, 18;
            17,   22, 27, 32;
            98,   94, 90, 86;
            114, 102, 90, 78;
        ];

        assert_eq!(CMatrix::mul(&a, &b), c);
        assert_eq!(CMatrix::mul(&b, &a), d);
    }
}

impl<const R: usize, const C: usize> Default for CMatrix<R, C> {
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<const R: usize, const C: usize> Index<usize> for CMatrix<R, C> {
    type Output = [f32; R];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for CMatrix<R, C> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

impl<const R: usize, const C: usize> fmt::Debug for CMatrix<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}", R, C)?;
        for y in 0..C {
            for x in 0..R {
                write!(f, "{} ", self[y][x])?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl<const R: usize, const C: usize> PartialEq for CMatrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

use std::iter::{FromIterator, IntoIterator};
use std::ops::{Add, Mul, Sub};
use std::slice::Iter;

/// Matrix4 is a 4x4 matrix stored in row-major format
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix4 {
    mat: [f32; 16],
}

impl Matrix4 {
    /// Return the zero matrix
    pub fn zero() -> Matrix4 {
        Matrix4 { mat: [0f32; 16] }
    }

    /// Return the identity matrix
    pub fn identity() -> Matrix4 {
        Matrix4 {
            mat: [
                1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32,
                0f32, 1f32,
            ],
        }
    }

    /// Create the matrix using the values passed
    pub fn new(mat: [f32; 16]) -> Matrix4 {
        Matrix4 { mat }
    }

    /// Access the element at row `i` column `j`
    pub fn at(&self, i: usize, j: usize) -> &f32 {
        &self.mat[4 * i + j]
    }

    /// Mutably access the element at row `i` column `j`
    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        &mut self.mat[4 * i + j]
    }

    /// Compute and return the transpose of this matrix
    #[must_use]
    pub fn transpose(&self) -> Matrix4 {
        let mut res = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(j, i);
            }
        }
        res
    }

    /// Compute and return the inverse of this matrix
    #[must_use]
    pub fn inverse(&self) -> Matrix4 {
        //MESA's matrix inverse, tweaked for row-major matrices
        let mut inv = Matrix4::zero();
        inv.mat[0] = self.mat[5] * self.mat[10] * self.mat[15]
            - self.mat[5] * self.mat[11] * self.mat[14]
            - self.mat[9] * self.mat[6] * self.mat[15]
            + self.mat[9] * self.mat[7] * self.mat[14]
            + self.mat[13] * self.mat[6] * self.mat[11]
            - self.mat[13] * self.mat[7] * self.mat[10];

        inv.mat[4] = -self.mat[4] * self.mat[10] * self.mat[15]
            + self.mat[4] * self.mat[11] * self.mat[14]
            + self.mat[8] * self.mat[6] * self.mat[15]
            - self.mat[8] * self.mat[7] * self.mat[14]
            - self.mat[12] * self.mat[6] * self.mat[11]
            + self.mat[12] * self.mat[7] * self.mat[10];

        inv.mat[8] = self.mat[4] * self.mat[9] * self.mat[15]
            - self.mat[4] * self.mat[11] * self.mat[13]
            - self.mat[8] * self.mat[5] * self.mat[15]
            + self.mat[8] * self.mat[7] * self.mat[13]
            + self.mat[12] * self.mat[5] * self.mat[11]
            - self.mat[12] * self.mat[7] * self.mat[9];

        inv.mat[12] = -self.mat[4] * self.mat[9] * self.mat[14]
            + self.mat[4] * self.mat[10] * self.mat[13]
            + self.mat[8] * self.mat[5] * self.mat[14]
            - self.mat[8] * self.mat[6] * self.mat[13]
            - self.mat[12] * self.mat[5] * self.mat[10]
            + self.mat[12] * self.mat[6] * self.mat[9];

        inv.mat[1] = -self.mat[1] * self.mat[10] * self.mat[15]
            + self.mat[1] * self.mat[11] * self.mat[14]
            + self.mat[9] * self.mat[2] * self.mat[15]
            - self.mat[9] * self.mat[3] * self.mat[14]
            - self.mat[13] * self.mat[2] * self.mat[11]
            + self.mat[13] * self.mat[3] * self.mat[10];

        inv.mat[5] = self.mat[0] * self.mat[10] * self.mat[15]
            - self.mat[0] * self.mat[11] * self.mat[14]
            - self.mat[8] * self.mat[2] * self.mat[15]
            + self.mat[8] * self.mat[3] * self.mat[14]
            + self.mat[12] * self.mat[2] * self.mat[11]
            - self.mat[12] * self.mat[3] * self.mat[10];

        inv.mat[9] = -self.mat[0] * self.mat[9] * self.mat[15]
            + self.mat[0] * self.mat[11] * self.mat[13]
            + self.mat[8] * self.mat[1] * self.mat[15]
            - self.mat[8] * self.mat[3] * self.mat[13]
            - self.mat[12] * self.mat[1] * self.mat[11]
            + self.mat[12] * self.mat[3] * self.mat[9];

        inv.mat[13] = self.mat[0] * self.mat[9] * self.mat[14]
            - self.mat[0] * self.mat[10] * self.mat[13]
            - self.mat[8] * self.mat[1] * self.mat[14]
            + self.mat[8] * self.mat[2] * self.mat[13]
            + self.mat[12] * self.mat[1] * self.mat[10]
            - self.mat[12] * self.mat[2] * self.mat[9];

        inv.mat[2] = self.mat[1] * self.mat[6] * self.mat[15]
            - self.mat[1] * self.mat[7] * self.mat[14]
            - self.mat[5] * self.mat[2] * self.mat[15]
            + self.mat[5] * self.mat[3] * self.mat[14]
            + self.mat[13] * self.mat[2] * self.mat[7]
            - self.mat[13] * self.mat[3] * self.mat[6];

        inv.mat[6] = -self.mat[0] * self.mat[6] * self.mat[15]
            + self.mat[0] * self.mat[7] * self.mat[14]
            + self.mat[4] * self.mat[2] * self.mat[15]
            - self.mat[4] * self.mat[3] * self.mat[14]
            - self.mat[12] * self.mat[2] * self.mat[7]
            + self.mat[12] * self.mat[3] * self.mat[6];

        inv.mat[10] = self.mat[0] * self.mat[5] * self.mat[15]
            - self.mat[0] * self.mat[7] * self.mat[13]
            - self.mat[4] * self.mat[1] * self.mat[15]
            + self.mat[4] * self.mat[3] * self.mat[13]
            + self.mat[12] * self.mat[1] * self.mat[7]
            - self.mat[12] * self.mat[3] * self.mat[5];

        inv.mat[14] = -self.mat[0] * self.mat[5] * self.mat[14]
            + self.mat[0] * self.mat[6] * self.mat[13]
            + self.mat[4] * self.mat[1] * self.mat[14]
            - self.mat[4] * self.mat[2] * self.mat[13]
            - self.mat[12] * self.mat[1] * self.mat[6]
            + self.mat[12] * self.mat[2] * self.mat[5];

        inv.mat[3] = -self.mat[1] * self.mat[6] * self.mat[11]
            + self.mat[1] * self.mat[7] * self.mat[10]
            + self.mat[5] * self.mat[2] * self.mat[11]
            - self.mat[5] * self.mat[3] * self.mat[10]
            - self.mat[9] * self.mat[2] * self.mat[7]
            + self.mat[9] * self.mat[3] * self.mat[6];

        inv.mat[7] = self.mat[0] * self.mat[6] * self.mat[11]
            - self.mat[0] * self.mat[7] * self.mat[10]
            - self.mat[4] * self.mat[2] * self.mat[11]
            + self.mat[4] * self.mat[3] * self.mat[10]
            + self.mat[8] * self.mat[2] * self.mat[7]
            - self.mat[8] * self.mat[3] * self.mat[6];

        inv.mat[11] = -self.mat[0] * self.mat[5] * self.mat[11]
            + self.mat[0] * self.mat[7] * self.mat[9]
            + self.mat[4] * self.mat[1] * self.mat[11]
            - self.mat[4] * self.mat[3] * self.mat[9]
            - self.mat[8] * self.mat[1] * self.mat[7]
            + self.mat[8] * self.mat[3] * self.mat[5];

        inv.mat[15] = self.mat[0] * self.mat[5] * self.mat[10]
            - self.mat[0] * self.mat[6] * self.mat[9]
            - self.mat[4] * self.mat[1] * self.mat[10]
            + self.mat[4] * self.mat[2] * self.mat[9]
            + self.mat[8] * self.mat[1] * self.mat[6]
            - self.mat[8] * self.mat[2] * self.mat[5];

        let mut det = self.mat[0] * inv.mat[0]
            + self.mat[1] * inv.mat[4]
            + self.mat[2] * inv.mat[8]
            + self.mat[3] * inv.mat[12];
        assert!(det != 0f32);
        det = 1f32 / det;

        for x in &mut inv.mat {
            *x *= det;
        }
        inv
    }

    /// Return an iterator over the matrix's elements. The iterator goes
    /// row by row through the matrix.
    pub fn iter(&self) -> Iter<f32> {
        self.mat.iter()
    }

    pub fn has_nans(&self) -> bool {
        for x in &self.mat {
            if x.is_nan() {
                return true;
            }
        }
        false
    }
}

impl FromIterator<f32> for Matrix4 {
    /// Create the matrix using the values from the iterator. The iterator should return
    /// the rows of the matrix one after another. The first 16 values returned will
    /// be used to set the matrix elements. If fewer than 16 values are returned the
    /// remaining entries will be 0
    fn from_iter<T: IntoIterator<Item = f32>>(it: T) -> Matrix4 {
        let mut m = Matrix4::zero();
        for (r, x) in m.mat.iter_mut().zip(it.into_iter()) {
            *r = x;
        }
        m
    }
}

impl<'a> FromIterator<&'a f32> for Matrix4 {
    /// Create the matrix using the values from the iterator. The iterator should return
    /// the rows of the matrix one after another. The first 16 values returned will
    /// be used to set the matrix elements. If fewer than 16 values are returned the
    /// remaining entries will be 0
    fn from_iter<T: IntoIterator<Item = &'a f32>>(it: T) -> Matrix4 {
        let mut m = Matrix4::zero();
        for (r, x) in m.mat.iter_mut().zip(it.into_iter()) {
            *r = *x;
        }
        m
    }
}

impl Add for Matrix4 {
    type Output = Matrix4;
    /// Add two matrices together
    fn add(self, rhs: Matrix4) -> Matrix4 {
        self.mat
            .iter()
            .zip(rhs.mat.iter())
            .map(|(&x, &y)| x + y)
            .collect()
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;
    /// Subtract two matrices
    fn sub(self, rhs: Matrix4) -> Matrix4 {
        self.mat
            .iter()
            .zip(rhs.mat.iter())
            .map(|(&x, &y)| x - y)
            .collect()
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;
    /// Multiply two matrices
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut res = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(i, 0) * *rhs.at(0, j)
                    + *self.at(i, 1) * *rhs.at(1, j)
                    + *self.at(i, 2) * *rhs.at(2, j)
                    + *self.at(i, 3) * *rhs.at(3, j);
            }
        }
        res
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;
    /// Multiply the matrix by a scalar
    fn mul(self, rhs: f32) -> Matrix4 {
        self.mat.iter().map(|&x| x * rhs).collect()
    }
}

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;
    /// Multiply the matrix by a scalar
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        rhs.mat.iter().map(|&x| x * self).collect()
    }
}

#[test]
fn test_add() {
    let mut a = Matrix4::identity();
    *a.at_mut(0, 1) = 1f32;
    let mut b = Matrix4::identity();
    *b.at_mut(2, 3) = 3f32;
    let c = Matrix4::new([
        2f32, 1f32, 0f32, 0f32, 0f32, 2f32, 0f32, 0f32, 0f32, 0f32, 2f32, 3f32, 0f32, 0f32, 0f32,
        2f32,
    ]);
    assert!(a + b == c);
}
#[test]
fn test_sub() {
    let mut a = Matrix4::identity();
    *a.at_mut(0, 1) = 1f32;
    let mut b = Matrix4::identity();
    *b.at_mut(2, 3) = 3f32;
    let c = Matrix4::new([
        0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -3f32, 0f32, 0f32, 0f32,
        0f32,
    ]);
    assert!(a - b == c);
}
#[test]
fn test_mul() {
    assert!(Matrix4::identity() * Matrix4::identity() == Matrix4::identity());
    let a = Matrix4::new([
        1f32, 2f32, 1f32, 0f32, 3f32, 1f32, 4f32, 2f32, 1f32, 2f32, -5f32, 4f32, 3f32, 2f32, 4f32,
        1f32,
    ]);
    let b = Matrix4::new([
        8f32, 0f32, 2f32, 3f32, -2f32, 1f32, 0f32, 1f32, 5f32, -2f32, 3f32, 1f32, 0f32, 0f32, 4f32,
        1f32,
    ]);
    let c = Matrix4::new([
        9f32, 0f32, 5f32, 6f32, 42f32, -7f32, 26f32, 16f32, -21f32, 12f32, 3f32, 4f32, 40f32,
        -6f32, 22f32, 16f32,
    ]);
    assert!(a * b == c);
}

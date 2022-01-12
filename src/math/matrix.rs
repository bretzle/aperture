use std::{
    ops::{Add, Mul, Sub},
    slice::Iter,
};

#[macro_export]
macro_rules! matrix {
	( $( $( $val:expr ),+ );* ; ) => {
		Matrix::new( [$( $( $val as f32 ),+ ),*] )
	};
}

/// Row major
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix {
    mat: [f32; 16],
}

impl Matrix {
    pub const IDENTITY: Self = Self {
        mat: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };

    pub fn zero() -> Self {
        Self { mat: [0.0; 16] }
    }

    pub fn new(mat: [f32; 16]) -> Self {
        Self { mat }
    }

    pub fn transpose(&self) -> Self {
        let mut res = Matrix::zero();
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(j, i);
            }
        }
        res
    }

    pub fn inverse(&self) -> Self {
        //MESA's matrix inverse, tweaked for row-major matrices
        let mut inv = Matrix::zero();
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

    pub fn iter(&self) -> Iter<f32> {
        self.mat.iter()
    }

    pub fn has_nans(&self) -> bool {
        todo!()
    }

    pub fn at(&self, i: usize, j: usize) -> &f32 {
        &self.mat[4 * i + j]
    }

    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        &mut self.mat[4 * i + j]
    }
}

impl FromIterator<f32> for Matrix {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut mat = Matrix::zero();
        for (r, x) in mat.mat.iter_mut().zip(iter.into_iter()) {
            *r = x;
        }
        mat
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Matrix {
        self.mat
            .iter()
            .zip(rhs.mat.iter())
            .map(|(&x, &y)| x + y)
            .collect()
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Matrix {
        self.mat
            .iter()
            .zip(rhs.mat.iter())
            .map(|(&x, &y)| x - y)
            .collect()
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut res = Matrix::zero();
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

impl Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f32) -> Matrix {
        self.mat.iter().map(|&x| x * rhs).collect()
    }
}

impl Mul<Matrix> for f32 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        rhs.mat.iter().map(|&x| x * self).collect()
    }
}

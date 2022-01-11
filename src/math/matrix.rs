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
        todo!()
    }

    pub fn inverse(&self) -> Self {
        todo!()
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

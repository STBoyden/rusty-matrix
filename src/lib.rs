#![allow(clippy::pedantic, incomplete_features, dead_code, unused_variables)]
#![feature(const_generics, const_evaluatable_checked, trait_alias)]

mod error;

use error::{Error, Result};
use std::{fmt::Debug, ops::*};

pub trait Numerical = Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + PartialEq
    + Sized
    + Copy
    + Debug
    + Default;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<T: Numerical, const X: usize, const Y: usize>
where
    [T; X * Y]: Sized,
{
    pub data: [T; X * Y],
    pub x_len: usize,
    pub y_len: usize,
}

impl<T: Numerical, const X: usize, const Y: usize> Add for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x + rhs.data[index])
            .collect();

        Matrix::new_from_vec(data).unwrap()
    }
}

impl<T: Numerical, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<Matrix<T, Z, W>> for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = Result<Matrix<T, Z, Y>>;

    fn mul(self, rhs: Matrix<T, Z, W>) -> Result<Matrix<T, Z, Y>> {
        if self.x_len != rhs.y_len {
            return Err(Error::NotEq(format!(
                "Number of columns in the left hand matrix ({}) should be equal to the \
                 amount of rows in the right hand matrix ({})",
                self.x_len, rhs.y_len
            )));
        }

        let mut data = [[T::default(); Z]; Y];

        data.iter_mut()
            .enumerate()
            .map(|(row_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(column_index, x)| {
                        *x = {
                            let mut cell = T::default();
                            for i in 0..rhs.y_len {
                                cell += self.get_at(i, row_index)
                                    * rhs.get_at(column_index, i);
                            }
                            cell
                        }
                    })
                    .last();
            })
            .last();

        let mat: Self::Output = Ok(Matrix::new(data));

        mat
    }
}

impl<T: Numerical, const X: usize, const Y: usize> Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    pub fn new(data: [[T; X]; Y]) -> Self {
        let mut dat: [T; X * Y] = [data[0][0]; X * Y];

        for i in 0..dat.len() {
            dat[i] = data[i / X][i % X];
        }

        Self {
            data: dat,
            x_len: X,
            y_len: Y,
        }
    }

    pub fn new_1d(data: [T; X * Y]) -> Self {
        Self {
            data,
            x_len: X,
            y_len: Y,
        }
    }

    fn new_from_vec(data: Vec<T>) -> Result<Self> {
        if data.len() != X * Y {
            return Err(Error::None);
        }

        let mut array: [T; X * Y] = [T::default(); X * Y];

        array
            .iter_mut()
            .enumerate()
            .map(|(index, x)| *x = data[index])
            .last();

        println!("{:#?}", array);

        Ok(Self {
            data: array,
            x_len: X,
            y_len: Y,
        })
    }

    pub fn get_at(&self, x: usize, y: usize) -> T {
        *self
            .data
            .get(x * self.x_len + y)
            .expect(&format!("Index ({}, {}) is out of range", x, y))
    }

    pub fn get_row_at(&self, y: usize) -> &[T] {
        &self.data[y % self.x_len..(y % self.x_len) + self.x_len]
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn index_check() {
        let mat1 = Matrix::new([[100, 200], [300, 400]]);

        assert_eq!(mat1.get_at(1, 0), 300);
    }

    #[test]
    fn add_check() {
        let mat1 = Matrix::new([[1, 2], [3, 4]]);
        let mat2 = Matrix::new([[1, 2], [3, 4]]);

        assert_eq!(mat1.clone() + mat2.clone(), Matrix::new([[2, 4], [6, 8]]));
        assert_ne!(mat1 + mat2, Matrix::new([[10, 10], [10, 10]]));
    }

    #[test]
    fn mul_check() {
        let mat1 = Matrix::new([[1, 2], [3, 4], [5, 6]]);
        let mat2 = Matrix::new([[1, 2, 3], [4, 5, 6]]);

        let res = mat1 * mat2;

        assert_eq!(res, Ok(Matrix::new([[0; 3]; 3])))
    }
}

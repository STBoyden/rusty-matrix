use crate::{
    common::Numerical,
    error::{Error, Result},
    matrix::Matrix,
};
use std::{fmt::Debug, ops::*};

#[derive(Debug, Copy, Clone, PartialEq)]
/// An implementation of the Matrix trait where the inner data is allocated on
/// the stack.
pub struct StackMatrix<T: Numerical, const X: usize, const Y: usize>
where
    [T; X * Y]: Sized,
{
    pub data: [T; X * Y],
    pub x_len: usize,
    pub y_len: usize,
}

impl<T: Numerical, const X: usize, const Y: usize> Add for StackMatrix<T, X, Y>
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

        StackMatrix::new_from_vec(&data).unwrap()
    }
}

impl<T: Numerical, const X: usize, const Y: usize> Sub for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x - rhs.data[index])
            .collect();

        StackMatrix::new_from_vec(&data).unwrap()
    }
}

impl<T: Numerical, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<StackMatrix<T, Z, W>> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = Result<StackMatrix<T, Z, Y>>;

    fn mul(self, rhs: StackMatrix<T, Z, W>) -> Self::Output {
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
                                cell += self.get_at_unchecked(row_index, i)
                                    * rhs.get_at_unchecked(i, column_index);
                            }
                            cell
                        }
                    })
                    .last();
            })
            .last();

        Ok(StackMatrix::new(data))
    }
}

impl<T: Numerical, const X: usize, const Y: usize> StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    /// Takes a trivially copyable 2-dimensional array, converts it into a
    /// 1-dimensional array which the Matrix type uses as it's internal
    /// data.
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

    /// Takes a trivially copyable 1-dimensional array which is used by the
    /// Matrix's inner data.
    pub fn new_1d(data: [T; X * Y]) -> Self {
        Self {
            data,
            x_len: X,
            y_len: Y,
        }
    }

    /// Takes a &[T] which is converted to a 1-dimensional trivially
    /// copyable array which is used by the Matrix's inner data.
    pub fn new_from_vec(data: &[T]) -> Result<Self> {
        if data.len() != X * Y {
            return Err(Error::IncorrectLength);
        }

        let mut array: [T; X * Y] = [T::default(); X * Y];

        array
            .iter_mut()
            .enumerate()
            .map(|(index, x)| *x = data[index])
            .last();

        Ok(Self {
            data: array,
            x_len: X,
            y_len: Y,
        })
    }
}

impl<T: Numerical, const X: usize, const Y: usize> Matrix<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn get_data(&self) -> &[T] { &self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }
}

use crate::{
    common::Numeric,
    error::{Error, Result},
    implementations::StackMatrix,
    matrix::Matrix,
};
use std::{fmt::Debug, ops::*};

#[derive(Debug, Clone, PartialEq)]
/// An implementation of the Matrix trait where the inner data is allocated on
/// the heap.
pub struct HeapMatrix<T: Numeric> {
    pub data: Vec<T>,
    pub x_len: usize,
    pub y_len: usize,
}

impl<T: Numeric> Add for HeapMatrix<T> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x + rhs.data[index])
            .collect();

        Ok(Self::new(&data, self.x_len, self.y_len))
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn add(self, rhs: StackMatrix<T, X, Y>) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x + rhs.data[index])
            .collect();

        Ok(Self::new(&data, self.x_len, self.y_len))
    }
}

impl<T: Numeric> Sub for HeapMatrix<T> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x - rhs.data[index])
            .collect();

        Ok(Self::new(&data, self.x_len, self.y_len))
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn sub(self, rhs: StackMatrix<T, X, Y>) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, x)| x - rhs.data[index])
            .collect();

        Ok(Self::new(&data, self.x_len, self.y_len))
    }
}

impl<T: Numeric> Mul for HeapMatrix<T> {
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.x_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let mut data = Vec::new();
        let mut data_inner = Vec::new();
        data_inner.resize(rhs.x_len, T::default());
        data.resize(self.y_len, data_inner);

        data.iter_mut()
            .enumerate()
            .map(|(row_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(column_index, x)| {
                        *x = {
                            let mut cell = T::default();
                            for i in 0..rhs.y_len {
                                cell += self.get_at_unchecked(i, row_index)
                                    * rhs.get_at_unchecked(column_index, i);
                            }
                            cell
                        }
                    })
                    .last();
            })
            .last();

        Ok(Self::new_2d(data))
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: StackMatrix<T, X, Y>) -> Self::Output {
        if self.x_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let mut data = Vec::new();
        let mut data_inner = Vec::new();
        data_inner.resize(rhs.x_len, T::default());
        data.resize(self.y_len, data_inner);

        data.iter_mut()
            .enumerate()
            .map(|(row_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(column_index, x)| {
                        *x = {
                            let mut cell = T::default();
                            for i in 0..rhs.y_len {
                                cell += self.get_at_unchecked(i, row_index)
                                    * rhs.get_at_unchecked(column_index, i);
                            }
                            cell
                        }
                    })
                    .last();
            })
            .last();

        Ok(Self::new_2d(data))
    }
}

impl<T: Numeric> HeapMatrix<T> {
    /// Takes a &[T] which is converted to a 1-dimensional Vec<T> which is used
    /// by the HeapMatrix's inner data.
    pub fn new(data: &[T], columns: usize, rows: usize) -> Self {
        Self {
            data: data.to_vec(),
            x_len: columns,
            y_len: rows,
        }
    }

    /// Consumes a Vec<Vec<T>> which is converted to a 1-dimensional
    /// Vec<T> which is used by the HeapMatrix's inner data.
    pub fn new_2d(data: Vec<Vec<T>>) -> Self {
        let x_len = data.len();
        let y_len = data[0].len();

        let mut dat = Vec::new();
        dat.resize(x_len * y_len, T::default());

        for i in 0..dat.len() {
            dat[i] = data[i / x_len][i % x_len];
        }

        Self {
            data: dat,
            x_len,
            y_len,
        }
    }

    /// Takes a trivially copyable 2-dimensional array, converts it into a
    /// 1-dimensional Vec<T> which the HeapMatrix type uses as it's internal
    /// data.
    pub fn new_owned_2d<const X: usize, const Y: usize>(data: [[T; X]; Y]) -> Self
    where
        [T; X * Y]: Sized,
    {
        let mut dat: Vec<T> = [data[0][0]; X * Y].to_vec();

        for i in 0..dat.len() {
            dat[i] = data[i / X][i % X];
        }

        Self {
            data: dat.to_vec(),
            x_len: X,
            y_len: Y,
        }
    }

    /// Takes a trivially copyable 1-dimensional array which is converted to a
    /// Vec<T> which is used as the HeapMatrix's inner data.
    pub fn new_owned_1d<const X: usize, const Y: usize>(data: [T; X * Y]) -> Self {
        Self {
            data: data.to_vec(),
            x_len: X,
            y_len: Y,
        }
    }

    /// Takes in a new row as a [T; X] and inserts it into the current
    /// HeapMatrix.
    pub fn insert_row<const X: usize>(&mut self, row: [T; X]) -> Result<()> {
        if row.len() != self.x_len {
            return Err(Error::IncorrectLength);
        }

        self.data.resize(self.data.len() + X, T::default());

        for (column_index, x) in row.iter().enumerate() {
            self.data[self.x_len * self.y_len + column_index] = *x;
        }

        self.y_len += 1;

        Ok(())
    }
}

impl<T: Numeric> Matrix<T> for HeapMatrix<T> {
    fn get_data(&self) -> &[T] { &self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }
}

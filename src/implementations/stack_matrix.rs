use crate::{
    common::Numeric,
    error::{Error, Result},
    implementations::HeapMatrix,
    matrix::Matrix,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::*,
};

#[derive(Debug, Copy, Clone, PartialEq)]
/// An implementation of the Matrix trait where the inner data is allocated on
/// the stack.
pub struct StackMatrix<T: Numeric, const X: usize, const Y: usize>
where
    [T; X * Y]: Sized,
{
    pub(crate) data: [T; X * Y],
    pub(crate) x_len: usize,
    pub(crate) y_len: usize,
}

impl<T: Numeric, const X: usize, const Y: usize> Add for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .map(|(index, x)| *x + rhs.data[index])
            .collect();

        Self::new_from_vec(&data).unwrap()
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn add(self, rhs: HeapMatrix<T>) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .map(|(index, x)| *x + rhs.data[index])
            .collect();

        Ok(HeapMatrix::new(&data, self.x_len, self.y_len))
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<&Self> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output { self + *rhs }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<&HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn add(self, rhs: &HeapMatrix<T>) -> Self::Output { self + rhs.clone() }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .map(|(index, x)| *x - rhs.data[index])
            .collect();

        Self::new_from_vec(&data).unwrap()
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn sub(self, rhs: HeapMatrix<T>) -> Self::Output {
        if self.x_len != rhs.x_len || self.y_len != rhs.y_len {
            return Err(Error::NotEq);
        }

        let data: Vec<T> = self
            .data
            .iter()
            .enumerate()
            .map(|(index, x)| *x - rhs.data[index])
            .collect();

        Ok(HeapMatrix::new(&data, X, Y))
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<&Self> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output { self - *rhs }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<&HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn sub(self, rhs: &HeapMatrix<T>) -> Self::Output { self - rhs.clone() }
}

impl<T: Numeric, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<StackMatrix<T, Z, W>> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = Result<StackMatrix<T, Z, Y>>;

    fn mul(self, rhs: StackMatrix<T, Z, W>) -> Self::Output {
        if self.x_len != rhs.y_len {
            return Err(Error::NotEq);
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
                                cell += self.get_at_unchecked(i, row_index)
                                    * rhs.get_at_unchecked(column_index, i);
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

impl<T: Numeric, const X: usize, const Y: usize> Mul<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: HeapMatrix<T>) -> Self::Output {
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

        Ok(HeapMatrix::new_2d(data))
    }
}

impl<T: Numeric, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<&StackMatrix<T, Z, W>> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = Result<StackMatrix<T, Z, Y>>;

    fn mul(self, rhs: &StackMatrix<T, Z, W>) -> Self::Output { self * *rhs }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<&HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: &HeapMatrix<T>) -> Self::Output { self * rhs.clone() }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x + rhs).collect();

        Self::new_from_vec(&data).unwrap()
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x - rhs).collect();

        Self::new_from_vec(&data).unwrap()
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x * rhs).collect();

        Self::new_from_vec(&data).unwrap()
    }
}

impl<T: Numeric, const X: usize, const Y: usize> StackMatrix<T, X, Y>
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
    pub const fn new_1d(data: [T; X * Y]) -> Self {
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

impl<T: Numeric, const X: usize, const Y: usize> PartialEq<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn eq(&self, other: &HeapMatrix<T>) -> bool {
        if X != other.x_len || Y != other.y_len {
            return false;
        }

        other.data == self.data
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Display for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_printable())
    }
}

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> Matrix<'a, T>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn get_data(&self) -> &[T] { &self.data }
    fn get_data_mut(&mut self) -> &mut [T] { &mut self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }
}

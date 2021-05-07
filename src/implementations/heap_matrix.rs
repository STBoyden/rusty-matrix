use crate::{
    common::Numeric,
    error::{Error, Result},
    implementations::StackMatrix,
    matrix::Matrix,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::*,
};

#[derive(Debug, Clone, PartialEq)]
/// An implementation of the Matrix trait where the inner data is allocated on
/// the heap.
pub struct HeapMatrix<T: Numeric> {
    pub(crate) data: Vec<T>,
    pub(crate) x_len: usize,
    pub(crate) y_len: usize,
}

impl<T: Numeric> Add for HeapMatrix<T> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output { self.mat_add(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn add(self, rhs: StackMatrix<T, X, Y>) -> Self::Output { self.mat_add(rhs) }
}

impl<'a, T: Numeric> Add<&Self> for HeapMatrix<T> {
    type Output = Result<Self>;

    fn add(self, rhs: &Self) -> Self::Output { self.mat_add(rhs.clone()) }
}

impl<T: Numeric, const X: usize, const Y: usize> Add<&StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn add(self, rhs: &StackMatrix<T, X, Y>) -> Self::Output { self.mat_add(*rhs) }
}

impl<T: Numeric> Sub for HeapMatrix<T> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output { self.mat_sub(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn sub(self, rhs: StackMatrix<T, X, Y>) -> Self::Output { self.mat_sub(rhs) }
}

impl<T: Numeric> Sub<&Self> for HeapMatrix<T> {
    type Output = Result<Self>;

    fn sub(self, rhs: &Self) -> Self::Output { self.mat_sub(rhs.clone()) }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<&StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn sub(self, rhs: &StackMatrix<T, X, Y>) -> Self::Output { self.mat_sub(*rhs) }
}

impl<T: Numeric> Mul for HeapMatrix<T> {
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output { self.mat_mul(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<HeapMatrix<T>>;

    fn mul(self, rhs: StackMatrix<T, X, Y>) -> Self::Output { self.mat_mul(rhs) }
}

impl<T: Numeric> Mul<&Self> for HeapMatrix<T> {
    type Output = Result<Self>;

    fn mul(self, rhs: &Self) -> Self::Output { self.mat_mul(rhs.clone()) }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<&StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    type Output = Result<Self>;

    fn mul(self, rhs: &StackMatrix<T, X, Y>) -> Self::Output { self.mat_mul(*rhs) }
}

impl<T: Numeric> Add<T> for HeapMatrix<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x + rhs).collect();

        Self::new(&data, self.x_len, self.y_len)
    }
}

impl<T: Numeric> Sub<T> for HeapMatrix<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x - rhs).collect();

        Self::new(&data, self.x_len, self.y_len)
    }
}

impl<T: Numeric> Mul<T> for HeapMatrix<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x * rhs).collect();

        Self::new(&data, self.x_len, self.y_len)
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

impl<T: Numeric, const X: usize, const Y: usize> PartialEq<StackMatrix<T, X, Y>>
    for HeapMatrix<T>
where
    [T; X * Y]: Sized,
{
    fn eq(&self, other: &StackMatrix<T, X, Y>) -> bool {
        if self.x_len != X || self.y_len != Y {
            return false;
        }

        self.data == other.data
    }
}

impl<T: Numeric> Display for HeapMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_printable())
    }
}

impl<'a, T: 'a + Numeric> Matrix<'a, T> for HeapMatrix<T> {
    fn get_data(&self) -> &[T] { &self.data }
    fn get_data_mut(&mut self) -> &mut [T] { &mut self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }

    fn mat_new(data: &[&[T]]) -> Result<Self> {
        let data: Vec<Vec<T>> = data.iter().map(|x| x.to_vec()).collect();

        Ok(Self::new_2d(data))
    }

    fn mat_new_1d(data: &[T], columns: usize, rows: usize) -> Result<Self> {
        Ok(Self::new(data, columns, rows))
    }

    fn mat_new_vec(data: Vec<Vec<T>>) -> Result<Self> { Ok(Self::new_2d(data)) }
}

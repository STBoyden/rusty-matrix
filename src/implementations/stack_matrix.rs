use crate::{common::Numeric, implementations::HeapMatrix, matrix::*};
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

impl<'a, T: 'a + Numeric, Mat, const X: usize, const Y: usize> Add<Mat>
    for StackMatrix<T, X, Y>
where
    Mat: Matrix<'a, T> + Sized,
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn add(self, rhs: Mat) -> Self::Output { self.mat_add(&rhs) }
}

// impl<'a, T: Numeric, const X: usize, const Y: usize> Add for StackMatrix<T,
// X, Y> where
//     [T; X * Y]: Sized,
// {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output { self.mat_add(&rhs) }
// }

// impl<T: Numeric, const X: usize, const Y: usize> Add<HeapMatrix<T>>
//     for StackMatrix<T, X, Y>
// where
//     [T; X * Y]: Sized,
// {
//     type Output = Self;

//     fn add(self, rhs: HeapMatrix<T>) -> Self::Output { self.mat_add(&rhs) }
// }

// impl<T: Numeric, const X: usize, const Y: usize> Add<&Self> for
// StackMatrix<T, X, Y> where
//     [T; X * Y]: Sized,
// {
//     type Output = Self;

//     fn add(self, rhs: &Self) -> Self::Output { self.mat_add(rhs) }
// }

// impl<T: Numeric, const X: usize, const Y: usize> Add<&HeapMatrix<T>>
//     for StackMatrix<T, X, Y>
// where
//     [T; X * Y]: Sized,
// {
//     type Output = Self;

//     fn add(self, rhs: &HeapMatrix<T>) -> Self::Output {
// self.mat_add(&rhs.clone()) } }

impl<T: Numeric, const X: usize, const Y: usize> Sub for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { self.mat_sub(&rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: HeapMatrix<T>) -> Self::Output { self.mat_sub(&rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<&Self> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output { self.mat_sub(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Sub<&HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: &HeapMatrix<T>) -> Self::Output { self.mat_sub(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<StackMatrix<T, Z, W>> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = StackMatrix<T, Z, Y>;

    fn mul(self, rhs: StackMatrix<T, Z, W>) -> Self::Output { self.mat_mul(&rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = HeapMatrix<T>;

    fn mul(self, rhs: HeapMatrix<T>) -> Self::Output { self.mat_mul(&rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize, const Z: usize, const W: usize>
    Mul<&StackMatrix<T, Z, W>> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
    [T; Z * W]: Sized,
    [T; Z * Y]: Sized,
{
    type Output = StackMatrix<T, Z, Y>;

    fn mul(self, rhs: &StackMatrix<T, Z, W>) -> Self::Output { self.mat_mul(rhs) }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<&HeapMatrix<T>>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = HeapMatrix<T>;

    fn mul(self, rhs: &HeapMatrix<T>) -> Self::Output { self.mat_mul(rhs) }
}

// impl<T: Numeric, const X: usize, const Y: usize> Add<T> for StackMatrix<T, X,
// Y> where
//     [T; X * Y]: Sized,
// {
//     type Output = Self;

//     fn add(self, rhs: T) -> Self::Output {
//         let data: Vec<T> = self.data.iter().map(|x| *x + rhs).collect();

//         Self::new_from_slice(&data)
//     }
// }

impl<T: Numeric, const X: usize, const Y: usize> Sub<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x - rhs).collect();

        Self::new_from_slice(&data)
    }
}

impl<T: Numeric, const X: usize, const Y: usize> Mul<T> for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|x| *x * rhs).collect();

        Self::new_from_slice(&data)
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
    pub fn new_from_slice(data: &[T]) -> Self {
        assert!(data.len() == X * Y);

        let mut array: [T; X * Y] = [T::default(); X * Y];

        array
            .iter_mut()
            .enumerate()
            .map(|(index, x)| *x = data[index])
            .last();

        Self {
            data: array,
            x_len: X,
            y_len: Y,
        }
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

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> MatrixAlloc<'a, T>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn mat_new(data: &[&[T]]) -> Self {
        assert!(data.len() == Y || data[0].len() == X);

        let mut array: [[T; X]; Y] = [[T::default(); X]; Y];

        array
            .iter_mut()
            .enumerate()
            .map(|(y_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(x_index, x)| {
                        *x = data[y_index][x_index];
                    })
                    .last()
            })
            .last();

        Self::new(array)
    }

    fn mat_new_1d(data: &[T], _columns: usize, _rows: usize) -> Self {
        Self::new_from_slice(data)
    }

    fn mat_new_vec(data: Vec<Vec<T>>) -> Self {
        assert!(data.len() == Y || data[0].len() == X);

        let mut array: [[T; X]; Y] = [[T::default(); X]; Y];

        array
            .iter_mut()
            .enumerate()
            .map(|(y_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(x_index, x)| {
                        *x = data[y_index][x_index];
                    })
                    .last()
            })
            .last();

        Self::new(array)
    }
}

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> MatrixRef<'a, T>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn get_data(&self) -> &[T] { &self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }
}

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> MatrixRef<'a, T>
    for &StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn get_data(&self) -> &[T] { &self.data }
    fn get_x_len(&self) -> usize { self.x_len }
    fn get_y_len(&self) -> usize { self.y_len }
}

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> Matrix<'a, T>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    fn get_data_mut(&mut self) -> &mut [T] { &mut self.data }
}

impl<'a, T: 'a + Numeric, const X: usize, const Y: usize> MatrixOp<'a, T>
    for StackMatrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
}

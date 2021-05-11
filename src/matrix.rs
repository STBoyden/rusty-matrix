use crate::{common::Numeric, error::*};
use std::ops::*;

pub trait MatrixRef<'a, T: Numeric> {
    /// Gets the Matrix's inner data as a &[T]
    fn get_data(&self) -> &[T];
    /// Gets the Matrix's x length
    fn get_x_len(&self) -> usize;
    /// Gets the Matrix's y length
    fn get_y_len(&self) -> usize;

    /// Gets the first item of the inner matrix data as a reference
    fn first(&self) -> Option<&T> { self.get_data().first() }
    /// Gets the last item of the inner matrix data as a reference
    fn last(&self) -> Option<&T> { self.get_data().last() }

    /// Takes x and y coordinates and returns a T if the coordinates are within
    /// the bounds of the Matrix, panics otherwise.
    fn get_at_unchecked(&self, x: usize, y: usize) -> T {
        assert!(x * y <= self.get_x_len() * self.get_y_len());
        *self
            .get_data()
            .get(y * self.get_x_len() + x)
            .unwrap_or_else(|| panic!("Index ({}, {}) is out of range", x, y))
    }

    /// Takes x and y coordinates and returns a Result<T>.
    fn get_at(&self, x: usize, y: usize) -> Result<T> {
        if let Some(coord_data) = self.get_data().get(y * self.get_x_len() + x) {
            Ok(*coord_data)
        } else {
            Err(Error::OutOfRange)
        }
    }

    /// Takes a y index to get a reference to the corresponding "row" of the
    /// inner matrix data.
    fn get_row_at(&self, row_index: usize) -> &[T] {
        &self.get_data()[row_index % self.get_x_len()
            ..(row_index % self.get_x_len()) + self.get_x_len()]
    }

    /// Formats the Matrix in a way that is easily printable.
    fn to_printable(&self) -> String {
        let mut out = String::new();
        for y in 0..self.get_y_len() {
            for x in 0..self.get_x_len() {
                out += &format!("{}\t", self.get_at_unchecked(x, y));
            }
            out += "\n";
        }

        out
    }
}

pub trait MatrixAlloc<'a, T: Numeric>: Matrix<'a, T> {
    /// Creates a new MatrixOpCapable<T> from the given 2d slice of data.
    fn mat_new(data: &[&[T]]) -> Self;
    /// Creates a new MatrixOpCapable<T> from the given 1d slice of data.
    fn mat_new_1d(data: &[T], columns: usize, rows: usize) -> Self;
    /// Creates a new MatrixOpCapable<T> from the given 2d Vec<T>.
    fn mat_new_vec(data: Vec<Vec<T>>) -> Self;
}

pub trait Matrix<'a, T: Numeric>: MatrixRef<'a, T> // + MatrixRefMut<'a, T>
{
    /// Gets the Matrix's inner data as a &mut [T]
    fn get_data_mut(&mut self) -> &mut [T];
    /// Gets the first item of the inner matrix data as a mutable reference
    fn first_mut(&mut self) -> Option<&mut T> { self.get_data_mut().first_mut() }
    /// Gets the last item of the inner matrix data as a mutable reference
    fn last_mut(&mut self) -> Option<&mut T> { self.get_data_mut().last_mut() }

    /// Takes a y index to get a mutable reference to the corresponding "row" of
    /// the inner matrix data
    fn get_row_at_mut(&mut self, row_index: usize) -> &mut [T] {
        let x_length = self.get_x_len();

        &mut self.get_data_mut()[row_index % x_length..(row_index % x_length) + x_length]
    }
}

pub trait MatrixScalarOp<'a, T: Numeric>:
    Matrix<'a, T> + MatrixAlloc<'a, T> + Sized + Add<T> + Sub<T> + Mul<T>
{
    fn scalar_add(&self, num: T) -> Self {
        let data: Vec<T> = self.get_data().iter().map(|x| *x + num).collect();

        Self::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }

    fn scalar_sub(&self, num: T) -> Self {
        let data: Vec<T> = self.get_data().iter().map(|x| *x - num).collect();

        Self::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }

    fn scalar_mul(&self, num: T) -> Self {
        let data: Vec<T> = self.get_data().iter().map(|x| *x * num).collect();

        Self::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }
}

pub trait MatrixOp<'a, T: Numeric>:
    Matrix<'a, T> + MatrixAlloc<'a, T> + Sized + Add + Sub + Mul + PartialEq
{
    fn mat_add<Other: Matrix<'a, T>>(&self, rhs: &Other) -> Self {
        assert!(
            self.get_x_len() == rhs.get_x_len() || self.get_y_len() == rhs.get_y_len()
        );

        let data: Vec<T> = self
            .get_data()
            .iter()
            .enumerate()
            .map(|(index, x)| *x + rhs.get_data()[index])
            .collect();

        Self::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }

    fn mat_sub<Other: Matrix<'a, T>>(&self, rhs: &Other) -> Self {
        assert!(
            self.get_x_len() == rhs.get_x_len() || self.get_y_len() == rhs.get_y_len()
        );

        let data: Vec<T> = self
            .get_data()
            .iter()
            .enumerate()
            .map(|(index, x)| *x - rhs.get_data()[index])
            .collect();

        Self::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }

    fn mat_mul<Other: Matrix<'a, T>, Res: MatrixOp<'a, T>>(&self, rhs: &Other) -> Res {
        assert!(self.get_x_len() * self.get_y_len() == rhs.get_x_len() * rhs.get_y_len());

        let mut data = Vec::new();
        data.resize(self.get_x_len() * self.get_y_len(), T::default());

        data.iter_mut()
            .enumerate()
            .map(|(index, x)| *x = self.get_data()[index] * rhs.get_data()[index])
            .last();

        Res::mat_new_1d(&data, self.get_x_len(), self.get_y_len())
    }

    fn dot_prod<Other: Matrix<'a, T>>(&self, rhs: &Other) -> Self {
        assert!(self.get_x_len() == rhs.get_y_len());

        let mut data = Vec::new();
        let mut data_inner = Vec::new();
        data_inner.resize(rhs.get_x_len(), T::default());
        data.resize(self.get_y_len(), data_inner);

        data.iter_mut()
            .enumerate()
            .map(|(row_index, y)| {
                y.iter_mut()
                    .enumerate()
                    .map(|(column_index, x)| {
                        *x = {
                            let mut cell = T::default();
                            for i in 0..rhs.get_y_len() {
                                cell += self.get_at_unchecked(i, row_index)
                                    * rhs.get_at_unchecked(column_index, i);
                            }
                            cell
                        }
                    })
                    .last();
            })
            .last();

        Self::mat_new_vec(data)
    }
}

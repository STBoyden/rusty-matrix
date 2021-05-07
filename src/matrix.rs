use crate::{common::Numeric, error::*};
use std::ops::*;

pub trait Matrix<'a, T: Numeric>:
    Sized + Add + Sub + Mul + Add<T> + Sub<T> + Mul<T> + PartialEq
where
    Self: 'a + Add<&'a Self> + Sub<&'a Self> + Mul<&'a Self>,
{
    /// Gets the Matrix's inner data as a &[T]
    fn get_data(&self) -> &[T];
    /// Gets the Matrix's inner data as a &mut [T]
    fn get_data_mut(&mut self) -> &mut [T];
    /// Gets the Matrix's x length
    fn get_x_len(&self) -> usize;
    /// Gets the Matrix's y length
    fn get_y_len(&self) -> usize;

    /// Creates a new Matrix<T> from the given 2d slice of data.
    fn mat_new(data: &[&[T]]) -> Result<Self>;
    /// Creates a new Matrix<T> from the given 1d slice of data.
    fn mat_new_1d(data: &[T], columns: usize, rows: usize) -> Result<Self>;
    /// Creates a new Matrix<T> from the given 2d Vec<T>.
    fn mat_new_vec(data: Vec<Vec<T>>) -> Result<Self>;

    /// Gets the first item of the inner matrix data as a reference
    fn first(&self) -> Option<&T> { self.get_data().first() }
    /// Gets the last item of the inner matrix data as a reference
    fn last(&self) -> Option<&T> { self.get_data().last() }
    /// Gets the first item of the inner matrix data as a mutable reference
    fn first_mut(&mut self) -> Option<&mut T> { self.get_data_mut().first_mut() }
    /// Gets the last item of the inner matrix data as a mutable reference
    fn last_mut(&mut self) -> Option<&mut T> { self.get_data_mut().last_mut() }

    /// Takes x and y coordinates and returns a T if the coordinates are within
    /// the bounds of the Matrix, panics otherwise.
    fn get_at_unchecked(&self, x: usize, y: usize) -> T {
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

    /// Takes a y index to get a mutable reference to the corresponding "row" of
    /// the inner matrix data
    fn get_row_at_mut(&mut self, row_index: usize) -> &mut [T] {
        let x_length = self.get_x_len();

        &mut self.get_data_mut()[row_index % x_length..(row_index % x_length) + x_length]
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

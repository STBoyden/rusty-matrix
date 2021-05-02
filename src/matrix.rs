use crate::{common::Numerical, error::*};
use std::ops::*;

pub trait Matrix<T: Numerical>: Sized + Add + Sub + Mul {
    /// Gets the Matrix's inner data as a &[T]
    fn get_data(&self) -> &[T];
    /// Gets the Matrix's x length
    fn get_x_len(&self) -> usize;
    /// Gets the Matrix's y length;
    fn get_y_len(&self) -> usize;

    /// Takes x and y coordinates and returns a Result<T>.
    fn get_at_unchecked(&self, x: usize, y: usize) -> T {
        *self
            .get_data()
            .get(x * self.get_x_len() + y)
            .unwrap_or_else(|| panic!("Index ({}, {}) is out of range", x, y))
    }

    /// Takes x and y coordinates and returns a T if the coordinates are within
    /// the bounds of the Matrix, panics otherwise.
    fn get_at(&self, x: usize, y: usize) -> Result<T> {
        if let Some(coord_data) = self.get_data().get(x * self.get_x_len() + y) {
            Ok(*coord_data)
        } else {
            Err(Error::OutOfRange)
        }
    }

    /// Takes a y index to get the corresponding row of the 1-dimensional inner
    /// array interpreted as a 2-dimensional array.
    fn get_row_at(&self, row_index: usize) -> &[T] {
        &self.get_data()[row_index % self.get_x_len()
            ..(row_index % self.get_x_len()) + self.get_x_len()]
    }
}

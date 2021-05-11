#![allow(
    clippy::pedantic,
    clippy::suspicious_arithmetic_impl,
    dead_code,
    incomplete_features
)]
#![feature(
    const_evaluatable_checked,
    const_fn,
    const_fn_trait_bound,
    const_generics,
    test,
    trait_alias
)]

extern crate test;

mod common;
mod error;
mod implementations;
mod matrix;
pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn stack_index_check() {
        let mat1 = StackMatrix::new([[100, 200], [300, 400]]);

        assert_eq!(mat1.get_at_unchecked(0, 1), 300);
    }

    #[test]
    fn stack_add_check() {
        let mat1 = StackMatrix::new([[1, 2], [3, 4]]);
        let mat2 = StackMatrix::new([[1, 2], [3, 4]]);

        assert_eq!(mat1 + mat2, StackMatrix::new([[2, 4], [6, 8]]));
        assert_ne!(mat1 + mat2, StackMatrix::new([[10, 10], [10, 10]]));
    }

    #[test]
    fn stack_sub_check() {
        let mat1 = StackMatrix::new([[100; 3]; 3]);
        let mat2 = StackMatrix::new([[25; 3]; 3]);

        assert_eq!(mat1 - mat2, StackMatrix::new([[75; 3]; 3]));
    }

    #[test]
    fn stack_mul_check() {
        let mat1 = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let mat2 = StackMatrix::new([[1, 2, 3], [4, 5, 6]]);

        let res = mat1 * mat2;

        assert_eq!(
            res,
            StackMatrix::new([[9, 12, 15], [19, 26, 33], [29, 40, 51]])
        );
    }

    #[test]
    fn heap_index_check() {
        let mat1 = HeapMatrix::new_owned_2d([[100, 200], [300, 400]]);

        assert_eq!(mat1.get_at_unchecked(0, 1), 300);
    }

    #[test]
    fn heap_add_check() {
        let mat1 = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);
        let mat2 = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(
            mat1.clone() + mat2.clone(),
            HeapMatrix::new_owned_2d([[2, 4], [6, 8]])
        );
        assert_ne!(mat1 + mat2, HeapMatrix::new_owned_2d([[10, 10], [10, 10]]));
    }

    #[test]
    fn heap_sub_check() {
        let mat1 = HeapMatrix::new_owned_2d([[100; 3]; 3]);
        let mat2 = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(mat1 - mat2, HeapMatrix::new_owned_2d([[75; 3]; 3]));
    }

    #[test]
    fn heap_mul_check() {
        let mat1 = HeapMatrix::new_owned_2d([[1, 2], [3, 4], [5, 6]]);
        let mat2 = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        let res = mat1.clone() * mat2.clone();

        assert_eq!(
            res,
            HeapMatrix::new_owned_2d([[9, 12, 15], [19, 26, 33], [29, 40, 51]])
        );
    }

    #[test]
    fn heap_insert_row_check() {
        let mut mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        mat.insert_row([5, 6]).unwrap();
        assert_eq!(mat, HeapMatrix::new_owned_2d([[1, 2], [3, 4], [5, 6]]));

        let res = mat.insert_row([7, 8, 9]);
        assert_eq!(res, Err(Error::IncorrectLength));
    }

    #[test]
    fn heap_stack_add_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(
            stack_mat + heap_mat.clone(),
            StackMatrix::new([[2, 4], [6, 8]])
        );
        assert_ne!(stack_mat + heap_mat, StackMatrix::new([[10, 10], [10, 10]]));
    }

    #[test]
    fn heap_stack_sub_check() {
        let stack_mat = StackMatrix::new([[100; 3]; 3]);
        let heap_mat = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(stack_mat - heap_mat, StackMatrix::new([[75; 3]; 3]));
    }

    #[test]
    fn heap_stack_mul_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        assert_eq!(
            stack_mat * heap_mat,
            HeapMatrix::new_owned_2d([[9, 12, 15], [19, 26, 33], [29, 40, 51]])
        );
    }

    #[test]
    fn heap_stack_inserted_row_add_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(
            stack_mat + heap_mat.clone(),
            StackMatrix::new([[2, 4], [6, 8]])
        );
    }

    #[test]
    fn heap_stack_inserted_row_sub_check() {
        let stack_mat = StackMatrix::new([[100; 3]; 3]);
        let heap_mat = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(stack_mat - heap_mat.clone(), StackMatrix::new([[75; 3]; 3]));
    }

    #[test]
    fn heap_stack_eq_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(stack_mat, heap_mat);
    }
}

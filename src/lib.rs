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
    use test::Bencher;

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

    #[bench]
    fn stack_add_bench(b: &mut Bencher) {
        let mat1 = StackMatrix::new([[1, 2], [3, 4]]);
        let mat2 = StackMatrix::new([[1, 2], [3, 4]]);

        b.iter(|| mat1 + mat2);
    }

    #[test]
    fn stack_sub_check() {
        let mat1 = StackMatrix::new([[100; 3]; 3]);
        let mat2 = StackMatrix::new([[25; 3]; 3]);

        assert_eq!(mat1 - mat2, StackMatrix::new([[75; 3]; 3]));
    }

    #[bench]
    fn stack_sub_bench(b: &mut Bencher) {
        let mat1 = StackMatrix::new([[100; 3]; 3]);
        let mat2 = StackMatrix::new([[25; 3]; 3]);

        b.iter(|| mat1 - mat2);
    }

    #[test]
    fn stack_mul_check() {
        let mat1 = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let mat2 = StackMatrix::new([[1, 2, 3], [4, 5, 6]]);

        let res = mat1 * mat2;

        assert_eq!(
            res,
            Ok(StackMatrix::new([[9, 12, 15], [19, 26, 33], [29, 40, 51]]))
        );
    }

    #[bench]
    fn stack_mul_bench(b: &mut Bencher) {
        let mat1 = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let mat2 = StackMatrix::new([[1, 2, 3], [4, 5, 6]]);

        b.iter(|| (mat1 * mat2).unwrap());
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
            Ok(HeapMatrix::new_owned_2d([[2, 4], [6, 8]]))
        );
        assert_ne!(
            mat1 + mat2,
            Ok(HeapMatrix::new_owned_2d([[10, 10], [10, 10]]))
        );
    }

    #[bench]
    fn heap_add_bench(b: &mut Bencher) {
        let mat1 = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);
        let mat2 = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        b.iter(|| mat1.clone() + &mat2);
    }

    #[test]
    fn heap_sub_check() {
        let mat1 = HeapMatrix::new_owned_2d([[100; 3]; 3]);
        let mat2 = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(mat1 - mat2, Ok(HeapMatrix::new_owned_2d([[75; 3]; 3])));
    }

    #[bench]
    fn heap_sub_bench(b: &mut Bencher) {
        let mat1 = HeapMatrix::new_owned_2d([[100; 2]; 2]);
        let mat2 = HeapMatrix::new_owned_2d([[25; 2]; 2]);

        b.iter(|| mat1.clone() - &mat2);
    }

    #[test]
    fn heap_mul_check() {
        let mat1 = HeapMatrix::new_owned_2d([[1, 2], [3, 4], [5, 6]]);
        let mat2 = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        let res = mat1.clone() * mat2.clone();

        assert_eq!(
            res,
            Ok(HeapMatrix::new_owned_2d([
                [9, 12, 15],
                [19, 26, 33],
                [29, 40, 51]
            ]))
        );
    }

    #[bench]
    fn heap_mul_bench(b: &mut Bencher) {
        let mat1 = HeapMatrix::new_owned_2d([[1, 2], [3, 4], [5, 6]]);
        let mat2 = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        b.iter(|| mat1.clone() * &mat2);
    }

    #[test]
    fn heap_insert_row_check() {
        let mut mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        mat.insert_row([5, 6]).unwrap();
        assert_eq!(mat, HeapMatrix::new_owned_2d([[1, 2], [3, 4], [5, 6]]));

        let res = mat.insert_row([7, 8, 9]);
        assert_eq!(res, Err(Error::IncorrectLength));
    }

    #[bench]
    fn heap_insert_row_bench(b: &mut Bencher) {
        let mut mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        b.iter(|| mat.insert_row([5, 6]).unwrap())
    }

    #[test]
    fn heap_stack_add_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(
            stack_mat + heap_mat.clone(),
            Ok(StackMatrix::new([[2, 4], [6, 8]]))
        );
        assert_ne!(
            stack_mat + heap_mat,
            Ok(StackMatrix::new([[10, 10], [10, 10]]))
        );
    }

    #[bench]
    fn heap_stack_add_bench(b: &mut Bencher) {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        b.iter(|| stack_mat + &heap_mat)
    }

    #[test]
    fn heap_stack_sub_check() {
        let stack_mat = StackMatrix::new([[100; 3]; 3]);
        let heap_mat = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(stack_mat - heap_mat, Ok(StackMatrix::new([[75; 3]; 3])));
    }

    #[bench]
    fn heap_stack_sub_bench(b: &mut Bencher) {
        let stack_mat = StackMatrix::new([[100; 3]; 3]);
        let heap_mat = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        b.iter(|| stack_mat - &heap_mat);
    }

    #[test]
    fn heap_stack_mul_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        assert_eq!(
            stack_mat * heap_mat,
            Ok(HeapMatrix::new_owned_2d([
                [9, 12, 15],
                [19, 26, 33],
                [29, 40, 51]
            ]))
        );
    }

    #[bench]
    fn heap_stack_mul_bench(b: &mut Bencher) {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let heap_mat = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        b.iter(|| stack_mat * &heap_mat);
    }

    #[test]
    fn heap_stack_inserted_row_add_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let mut heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(
            stack_mat + heap_mat.clone(),
            Ok(StackMatrix::new([[2, 4], [6, 8]]))
        );

        heap_mat.insert_row([5, 6]).unwrap();
        assert_eq!(stack_mat + heap_mat, Err(Error::NotEq));
    }

    #[test]
    fn heap_stack_inserted_row_sub_check() {
        let stack_mat = StackMatrix::new([[100; 3]; 3]);
        let mut heap_mat = HeapMatrix::new_owned_2d([[25; 3]; 3]);

        assert_eq!(
            stack_mat - heap_mat.clone(),
            Ok(StackMatrix::new([[75; 3]; 3]))
        );

        heap_mat.insert_row([25; 3]).unwrap();
        assert_eq!(stack_mat - heap_mat, Err(Error::NotEq));
    }

    #[test]
    fn heap_stack_inserted_row_mul_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4], [5, 6]]);
        let mut heap_mat = HeapMatrix::new_owned_2d([[1, 2, 3], [4, 5, 6]]);

        assert_eq!(
            stack_mat * heap_mat.clone(),
            Ok(HeapMatrix::new_owned_2d([
                [9, 12, 15],
                [19, 26, 33],
                [29, 40, 51]
            ]))
        );

        heap_mat.insert_row([7; 3]).unwrap();
        assert_eq!(stack_mat * heap_mat, Err(Error::NotEq));
    }

    #[test]
    fn heap_stack_eq_check() {
        let stack_mat = StackMatrix::new([[1, 2], [3, 4]]);
        let mut heap_mat = HeapMatrix::new_owned_2d([[1, 2], [3, 4]]);

        assert_eq!(stack_mat, heap_mat);

        heap_mat.insert_row([5, 6]).unwrap();
        assert_ne!(stack_mat, heap_mat);
    }
}

#![allow(
    clippy::pedantic,
    incomplete_features,
    dead_code,
    clippy::suspicious_arithmetic_impl
)]
#![feature(const_generics, const_evaluatable_checked, trait_alias)]

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

        assert_eq!(mat1.get_at_unchecked(1, 0), 300);
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
            Ok(StackMatrix::new([[9, 12, 15], [19, 26, 33], [29, 40, 51]]))
        )
    }


    }


    #[test]

    }

    #[test]
    }

    #[test]


    }
}

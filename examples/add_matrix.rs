#![allow(incomplete_features)]
#![feature(const_evaluatable_checked)]
use rusty_matrix::prelude::*;

fn main() {
    let stack_mat = StackMatrix::new([[100; 2]; 2]);
    let heap_mat = HeapMatrix::new_owned_2d([[200; 2]; 2]);

    println!(
        "{}\t\t+\n{}\t\t=\n{}",
        stack_mat,
        heap_mat.clone(),
        (stack_mat + heap_mat)
    );
}

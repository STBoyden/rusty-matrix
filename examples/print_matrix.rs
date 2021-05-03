use rusty_matrix::prelude::*;

fn main() {
    println!("{}", StackMatrix::new([[1, 2, 3]; 5]));
    println!("{}", StackMatrix::new([[7, 8, 9, 10]; 5]));

    let mut matrix = HeapMatrix::new_owned_2d([[100; 3]; 2]);
    println!("Before:\n{}", matrix);
    matrix.insert_row([200; 3]).unwrap();
    println!("After:\n{}", matrix);
}

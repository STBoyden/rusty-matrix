use rusty_matrix::Matrix;

fn main() {
    let mat1 = Matrix::new([[1, 2], [3, 4], [5, 6]]);
    let mat2 = Matrix::new([[1, 2, 3], [4, 5, 6]]);

    let res = mat1 * mat2;

    println!("{:?}", res);
}

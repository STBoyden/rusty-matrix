# Rusty Matrix
A generic matrix implementation for Rust.

Matrix implementations use the trait `Matrix<T>` and the default implementations are:

- `StackMatrix<T, X, Y>` - A stack-based matrix implementation where `T` is the type, `X` is the amount of columns and `Y` is the amount of rows. The advantage of this implementation is the ability to check mathematical operations at compile time against other `StackMatrix`'s.
- `HeapMatrix<T>` - A heap-based matrix implementation where `T` is the type of the matrix. Due to the limitations of the heap-based solution, it is not possible to check mathematical operations at compile time. However, `HeapMatrix`s are able to be grown and shrank (TODO) during runtime.

Regardless of which implementation you use, both implementations can use the mathematical operators on each other. For example:

```rust
let stack_mat = StackMatrix::new([1, 2], [3, 4], [5, 6]);
let heap_mat = HeapMatrix::new_owned_2d([1, 2, 3], [4, 5, 6]);

stack_mat * heap_mat // Is equal to:
                     //  Ok(HeapMatrix::new_owned_2d([
                     //      [9,  12, 15],
                     //      [19, 26, 33],
                     //      [29, 40, 51]
                     //  ]))
```

This crate requires a nightly version of Rust >= 1.53.

The [GitHub](https://github.com/STBoyden/rusty-matrix) repository is a mirror of my [self-hosted Gitea instance](https://git.stboyden.com/STBoyden/rusty-matrix).

#![allow(incomplete_features, unused_variables)]
#![feature(const_evaluatable_checked)]
use criterion::{criterion_group, criterion_main, Criterion};
use rusty_matrix::prelude::*;

fn benchmark(c: &mut Criterion) {
    let stack_mat_one = StackMatrix::new([[3; 100]; 100]);
    let stack_mat_two = StackMatrix::new([[4; 100]; 100]);
    let heap_mat_one = HeapMatrix::new_owned_2d([[3; 100]; 100]);
    let heap_mat_two = HeapMatrix::new_owned_2d([[4; 100]; 100]);

    c.bench_function("stack + stack", |b| {
        b.iter(|| stack_mat_one + stack_mat_two)
    });
    c.bench_function("stack * stack", |b| {
        b.iter(|| stack_mat_one * stack_mat_two)
    });
    c.bench_function("stack - stack", |b| {
        b.iter(|| stack_mat_two - stack_mat_one)
    });
    c.bench_function("stack dot stack", |b| {
        b.iter(|| stack_mat_one.dot_prod(&stack_mat_two))
    });
    c.bench_function("stack + heap", |b| {
        b.iter(|| stack_mat_one + heap_mat_two.clone())
    });
    c.bench_function("stack * heap", |b| {
        b.iter(|| stack_mat_one * heap_mat_two.clone())
    });
    c.bench_function("stack - heap", |b| {
        b.iter(|| stack_mat_two - heap_mat_one.clone())
    });
    c.bench_function("stack dot heap", |b| {
        b.iter(|| stack_mat_two.dot_prod(&heap_mat_one))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

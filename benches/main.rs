extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const SMALL_LEN: usize = 255;
const LARGE_LEN: usize = 4096;

fn equal_small(c: &mut Criterion) {
    let vec1 = black_box(vec![5u8; SMALL_LEN]);
    let vec2 = black_box(vec1.clone());

    c.bench_function("equal_small", |b| {
        b.iter(|| {
            black_box(&vec1 == &vec2);
        })
    });
}

fn equal_large(c: &mut Criterion) {
    let vec1 = vec![5u8; LARGE_LEN];
    let vec2 = vec1.clone();

    c.bench_function("equal_large", |b| {
        b.iter(|| {
            black_box(&vec1 == &vec2);
        })
    });
}

fn unequal_small(c: &mut Criterion) {
    let vec = vec![5u8; SMALL_LEN - 1];
    let mut vec1 = vec.clone();
    let mut vec2 = vec.clone();
    vec1.push(1);
    vec2.push(2);

    c.bench_function("unequal_small", |b| b.iter(|| black_box(&vec1 != &vec2)));
}

fn unequal_large(c: &mut Criterion) {
    let vec = vec![5u8; LARGE_LEN - 1];
    let mut vec1 = vec.clone();
    let mut vec2 = vec.clone();
    vec1.push(1);
    vec2.push(2);

    c.bench_function("unequal_large", |b| b.iter(|| black_box(&vec1 != &vec2)));
}

criterion_group!(
    benches,
    equal_small,
    equal_large,
    unequal_small,
    unequal_large
);
criterion_main!(benches);

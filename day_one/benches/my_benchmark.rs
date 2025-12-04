use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_one::find_total;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("AoC Day 1", |b| b.iter(|| find_total(black_box(include_str!("input")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
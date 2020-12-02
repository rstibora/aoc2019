use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_framework::year2019::calendar::day_12::simulate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day_12::simulate", |b| b.iter(|| simulate(black_box(1000000),
                     black_box(&mut vec![[0, 0, 0]; 3]), &mut vec![[1, 1, 1]; 3])));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
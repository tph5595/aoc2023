use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day04::{p1, p2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1", |b| b.iter(|| p1("./src/input.txt", false)));
    c.bench_function("p2 20", |b| b.iter(|| p2("./src/input.txt", false)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

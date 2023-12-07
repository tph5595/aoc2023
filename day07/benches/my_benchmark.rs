use criterion::{criterion_group, criterion_main, Criterion};
use day07::{p1, p2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1", |b| b.iter(|| p1("./src/input.txt")));
    c.bench_function("p2", |b| b.iter(|| p2("./src/input.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

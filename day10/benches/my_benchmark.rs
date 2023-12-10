use criterion::{criterion_group, criterion_main, Criterion};
use day10::{p1, p2, p2_og_idea};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1", |b| b.iter(|| p1("./src/input.txt")));
    c.bench_function("p2", |b| b.iter(|| p2("./src/input.txt")));
    c.bench_function("p2 orginal idea", |b| b.iter(|| p2_og_idea("./src/input.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

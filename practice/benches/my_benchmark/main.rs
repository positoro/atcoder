use criterion::{criterion_group, criterion_main, Criterion};
use my_lib_for_bench::a;

fn bm1(c: &mut Criterion) {
    c.bench_function("plus", |b| b.iter(|| a::for_test_bench_a()));
}

fn bm2(c: &mut Criterion) {
    c.bench_function("multiple", |b| b.iter(|| a::for_test_bench_b()));
}
criterion_group!(benches, bm1, bm2);
criterion_main!(benches);

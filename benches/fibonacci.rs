use criterion::{Criterion, criterion_group, criterion_main};
use leetcode_rust::fibonacci::Solution;

fn criterion_bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| Solution::fibonacci(20)));
}


criterion_group!(benches, criterion_bench_fibonacci);
criterion_main!(benches);


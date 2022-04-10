use criterion::{Criterion, criterion_group, criterion_main};
use leetcode_rust::two_sum::Solution;

fn criterion_bench_two_sum(c: &mut Criterion) {
    c.bench_function("bench_two_sum", |b| b.iter(|| Solution::two_sum(vec![2, 7, 11, 15], 9)));
}

fn criterion_bench_two_sum_hash(c: &mut Criterion) {
    c.bench_function("bench_two_sum_hash", |b| b.iter(|| Solution::two_sum_hash(vec![2, 7, 11, 15], 9)));
}

criterion_group!(benches, criterion_bench_two_sum, criterion_bench_two_sum_hash);
criterion_main!(benches);


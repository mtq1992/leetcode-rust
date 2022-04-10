# leetcode-rust

src目录结构
* 每个算法题一个文件。包含算法实现和部分测试和Benchmark。
* lib.rs 文件 声明每个算法文件。


# 常用命令

## 测试

`cargo test --lib two_sum`

```
➜  leetcode-rust git:(master) ✗ cargo test --lib two_sum
   Compiling leetcode-rust v0.1.0 (/Users/matianqi/Project/Rust/leetcode-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.49s
     Running unittests src/lib.rs (target/debug/deps/leetcode_rust-3503d8f3d79d51f2)

running 3 tests
test two_sum::tests::test_2 ... ok
test two_sum::bench::bench_two_sum ... ok
test two_sum::tests::test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
## Benchmark

### rust 自带 bench
使用官方 Benchmark 需要安装 nightly 版本rust。

安装命令  
`rustup install nightly`

在项目根目录下切换成nightly  
`rustup override set nightly`

`cargo bench --lib two_sum`

```
➜  leetcode-rust git:(master) ✗ cargo bench --lib two_sum
   Compiling leetcode-rust v0.1.0 (/Users/matianqi/Project/Rust/leetcode-rust)
    Finished bench [optimized] target(s) in 0.66s
     Running unittests src/lib.rs (target/release/deps/leetcode_rust-74154c4bc1045562)

running 3 tests
test two_sum::tests::test ... ignored
test two_sum::tests::test_2 ... ignored
test two_sum::bench::bench_two_sum ... bench:         190 ns/iter (+/- 23)

test result: ok. 0 passed; 0 failed; 2 ignored; 1 measured; 0 filtered out; finished in 0.34s

```

### Criterion

需要在 Cargo.toml 添加如下内容  
```
[[bench]]
name = "two_sum"
harness = false

[[bench]]
name = "three_sum"
harness = false

```  
多个 `bench` 声明多个目标。  
`harness = false` 这里告诉编译器关闭 libtest benchmark  

`target/criterion/report/index.html` 是生成的 benchmark 报告。

```
➜  leetcode-rust git:(master) ✗ cargo bench --bench three_sum 
    Finished bench [optimized] target(s) in 0.06s
     Running benches/three_sum.rs (target/release/deps/three_sum-e14d6d8ac6870b43)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
bench_two_sum           time:   [190.17 ns 191.89 ns 193.70 ns]                          
                        change: [+0.3406% +1.8265% +3.2828%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

bench_two_sum_hash      time:   [388.30 ns 398.36 ns 413.84 ns]                               
                        change: [-2.5244% -0.5570% +2.0467%] (p = 0.64 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

```

# 参考
[基准测试 benchmark](https://course.rs/test/benchmark.html)   
[Aloxaf/LeetCode-Rust](https://github.com/Aloxaf/LeetCode-Rust)   
[Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/user_guide/migrating_from_libtest.html)
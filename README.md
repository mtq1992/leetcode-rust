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

# 参考
[基准测试 benchmark](https://course.rs/test/benchmark.html)  
[Aloxaf/LeetCode-Rust](https://github.com/Aloxaf/LeetCode-Rust)
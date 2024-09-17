+++
title = "测量排序性能 | Measure Sort Performance"
date = "2024-09-07"

[taxonomies]
tags = ["array", "sort", "bench", "test"]
+++

通过在一组精心设计的数据上进行排序来度量排序算法的性能

<!-- more -->

对上次 [二分搜索 | Binary Search](@/posts/binary-search/index.md) `bench` 设计缺陷的遗憾，
将在本次 [测量排序性能 | Measure Sort Performance](@/posts/sort/measure-sort-performance.md) 中得到弥补。

# 随机数组生成

代码定义在 [test_data.rs](../src/test_data.rs)。主要有以下几个函数。

```rust
fn gen_random<T, R, const N: usize>(range: R) -> [T; N]
fn gen_stroll<T, R, const N: usize>(start: T, stride: R) -> [T; N]
fn gen_function<T, const N: usize>(function: impl FnMut(usize) -> T) -> [T; N]
fn merge_array<T: AddAssign, const N: usize>(lhs: [T; N], rhs: [T; N]) -> [T; N]
fn with_noise<T, R, const N: usize>(array: [T; N], noise: R) -> [T; N]
```

以 `gen_random` 为例。

````rust
/// Generates an array of random values in the given range.
///
/// # Examples
///
/// ```
/// use sort::test_data::gen_random;
/// let range = -5..10;
/// let array: [i8; 100] = gen_random(range.clone());
/// assert!(array.iter().all(|x| range.contains(x)));
/// ```
pub fn gen_random<T, R, const N: usize>(range: R) -> [T; N]
where
    T: SampleUniform,
    R: SampleRange<T> + Clone,
{
    let mut rng = thread_rng();
    let uninit_array = MaybeUninit::<[T; N]>::uninit();

    unsafe {
        let mut array = uninit_array.assume_init();
        array
            .iter_mut()
            .for_each(|x| *x = rng.gen_range(range.clone()));
        array
    }
}
````

- 函数接受一个范围 `R`，比如 `-5..10` 表示 `-5` 到 `10` 不包含 `10` 的区间。
- 生成一个随机数组 `[T; N]`，长度为 `N`，元素类型为 `T`，
  生成的元素都在范围 `range` 中，数组长度和元素类型均可自动推断。

`rust` 的泛型使用起来非常优雅。
对于调用者只需要标注接受的数组大小即可获得一个栈上的固定长度数组，
甚至数组的大小也可以由后面的使用自动推断。

```rust
// 生成 10 个元素的数组，元素的类型由 -5..10 和后面的使用推断
let array: [_; 10] = gen_random(-5..10);
```

对于代码的实现，先创建一个数组，然后使用 `for` 循环将数组中的每个元素都随机生成。
这里使用了 `MaybeUninit` 来创建一个未初始化的数组，来避免不必要的初始化开销。

# 使用固定的数组

上次测试 `binary-search` 时每次 bench 时生成随机数组的方案显然是不可取的，
每次测试的数据不一致，得到的结果也不一致，并且对于后面的分析也无益。

通过在 `src/bin/*.rs` 中的一些小程序调用 [test_data.rs](../src/test_data.rs)
中的一些函数生成随机数组并保存到文件，可以使用 `gen-bench-data.sh` 脚本一键生成测试数据。

```sh
#!/bin/bash

# 要生成的数据
commands=(
  gen-gaussian-with-noise
  gen-high-sample-sin-with-noise
  gen-low-sample-sin-with-noise
  gen-random
  gen-stroll
  gen-trend-increasing
)

cmd_pids=()

# 生成的目标文件夹
mkdir -p ./bench-data

# 并行生成测试数据
for cmd in "${commands[@]}"; do
  output_file="./bench-data/${cmd#gen-}.json"
  cargo run --bin="$cmd" >"$output_file" 2>/dev/null || exit 1 &
  cmd_pids+=($!)
done

# 等待所有进程正常退出，否则错误退出
for pid in "${cmd_pids[@]}"; do
  wait "$pid" || exit 101
done
```

将生成的测试数据放在了 `sort/bench-data/*.json` 中

```console
文件大小 ： 元素个数 ： 文件名
5.8k  1k gaussian-with-noise.json
6.5k  1k high-sample-sin-with-noise.json
6.5k  1k low-sample-sin-with-noise.json
 59k 10k random.json
 70k 10k stroll.json
5.7k  1k trend-increasing.json
```

至于为什么测试的数组长度只有这么点，由于数组是在栈上的， 而 `main` 线程一般被分配 8MB
的栈空间，其他线程默认配置了 2MB 空间（虽然可以修改），如果数组过长，会导致栈溢出。😢
并且由于测试数据要提交到 git 仓库中，还要在文件大小之间权衡。

以下是自动生成的测试数据的图表。

<div id="graph">
  <script src="../graph.js" defer></script>
  <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js"></script>
  <div class="chart-container">
    <div id="graph-random" class="chart large-chart"></div>
    <div id="graph-stroll" class="chart large-chart"></div>
    <div id="graph-trend-increasing" class="chart small-chart"></div>
    <div id="graph-gaussian-with-noise" class="chart small-chart"></div>
    <div id="graph-low-sample-sin-with-noise" class="chart small-chart"></div>
    <div id="graph-high-sample-sin-with-noise" class="chart small-chart"></div>
  </div>
  <style>
    .chart-container { display: flex; flex-wrap: wrap; }
    .chart { margin-bottom: 30px; }
    .chart.large-chart { flex: 1 1 100%; height: 400px; }
    .chart.small-chart { flex: 1 1 50%; height: 300px; }
    @media (max-width: 600px) {
      .chart.small-chart { flex: 1 1 100%; } 
    }
  </style>
</div>

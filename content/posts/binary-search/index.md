+++
title = "二分搜索 | Binary Search"
date = "2024-09-05"

[taxonomies]
tags = ["search", "array"]
+++

使用二分搜索算法在有序数组中查找目标值。

<!-- more -->

```rust
let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
```

# 逐一比较

对于一个数组，要从中寻找其中某一个数字的索引，我们可能最简单的想法是逐一比较 :)

```rust
fn sequential_search<T: Eq>(target: &T, slice: &[T]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .find_map(|(i, x)| if x == target { Some(i) } else { None })
}
```

先来看看这个函数签名，这个函数接受参数

- `target: &T`：要寻找的目标的引用，目标的类型为 `T`，
- `slice: &[T]`：待寻找的数组切片，元素的类型为 `T`。
- `-> Option<usize>`：函数没有找到返回 `None`，找到了返回 `Some(i)`，其中 `i` 是目标的索引。
- `<T: Eq>`：标注了 `T` 类型必须实现 `Eq` trait，也就是可以判断任意两个 `T` 是否相等。

而函数体一个迭代器，对于这个切片的每一个元素`x`

- enumerate 返回 `(i, x)`，其中 `i` 是元素的索引,`x` 是元素的值。
- find_map 会寻找第一个满足要求的 `(i, x)`，如果没有找到，返回 `None`。

相当于循环的写法

```rust
for i in 0..slice.len() {
    if slice[i] == target {
        return Some(i);
    }
}
None
```

再为其添加一点测试代码，测试代码在 `src/main.rs::test::sequential_search` 中

```sh
s@xj63 ..tion-Notes/content/posts/binary-search (git)-[main] % cargo test sequential
test tests::sequential_search::find ... ok
test tests::sequential_search::no_such_element ... ok
test tests::sequential_search::slice_is_empty ... ok
test tests::sequential_search::random_check ... ok
```

一个非常简单的逐一比较的就完成了

# 并行比较

你可能会觉得这样太慢了，我的 `cpu` 有这么多核心，不应该“一核有难，多核围观”，可以使用 `rayon` 给他加入一点点的并行。

```rust
fn parallel_search<T: Eq + Sync>(target: &T, slice: &[T]) -> Option<usize> {
    use rayon::prelude::*;

    slice
        .par_iter()
        .enumerate()
        .find_map_any(|(i, x)| if x == target { Some(i) } else { None })
}
```

这里的 `par_iter` 就是 `rayon` 的一些小操作来并行的迭代这个切片，调包真的很快乐。

但是使用了 `find_map_any` 并不保证找到的是这个数组中第一个出现的，
也就是说如果这个数组中存在相同多个元素，找到的不能确定是哪一个，
如果需要找第一个，可以使用 `find_map_first`。

# 二分搜索

废话了这么多，终于到了本集的主角： `Binary Search`

数据中的单调递增特性不好好利用有些浪费

## 递归形式

```rust
fn binary_search<T: Ord>(target: &T, slice: &[T]) -> Option<usize> {
    let (left, middle, right) = split_at_middle(slice);
    Some(match Ord::cmp(target, middle?) {
        Ordering::Less => binary_search(target, left)?,
        Ordering::Equal => left.len(),
        Ordering::Greater => left.len() + 1 + binary_search(target, right)?,
    })
}
```

可以说相当的漂亮

- `split_at_middle` 会将切片从中间划分为三部分，`left` 为前半部分，`middle` 为中间元素，`right` 为后半部分
- 将 `target` 和 `middle` 比较：
  - `target < middle`: 说明 `target` 可能在 `left` 中，在 `left` 中继续二分搜索。
  - `target = middle`: 那就找到了，元素的索引为左侧数组的长度。
  - `target > middle`: 说明 `target` 可能在 `right` 中，那就在 `right` 中继续二分搜索，还应当加上从 `slice` 被分出去带来的偏移量。

`split_at_middle` 的实现在这里：

```rust
fn split_at<T>(slice: &[T], index: usize) -> (&[T], Option<&T>, &[T]) {
    if index >= slice.len() {
        return (&[], None, &[]);
    }
    (&slice[..index], Some(&slice[index]), &slice[index + 1..])
}

fn split_at_middle<T>(slice: &[T]) -> (&[T], Option<&T>, &[T]) {
    let mid = slice.len() / 2;
    split_at(slice, mid)
}
```

## 标准库中的实现

标准库中的二分搜索实现参考
[doc](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
[src](https://doc.rust-lang.org/1.80.1/src/core/slice/mod.rs.html#2740-2830)

```rust
fn std_binary_search<T: Ord>(target: &T, slice: &[T]) -> Option<usize> {
    match slice.binary_search(target) {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}
```

# 一元一次函数搜索

<div id="graph-random-array">
  <script src="./graph-random-array.js" defer></script>
  <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js"></script>
  <div class="chart-container" style="position: relative; height: 400px; overflow: hidden;"></div>
</div>

这是使用随机数生成的一个1000个元素的数组，然后将数组排序。
可以看到在总体上是一条直线，而这个特征随着元素的增多可能会更明显。
利用这个特征，我们可以简单的估算目标值的索引，
从而可能更快的找到目标值（当然现实中没有这样的好事）。

```rust
use num_traits::Num;

/// linear_search for an element in a slice
#[allow(dead_code)]
fn linear_search<T>(target: &T, slice: &[T]) -> Option<usize>
where
    T: Ord + Num + Into<f64> + Copy + Clone + std::fmt::Debug,
{
    let first = slice.first()?;
    let last = slice.last()?;

    if first == last {
        return if first == target { Some(0) } else { None };
    }

    let d: f64 = Into::<f64>::into(*last) - Into::<f64>::into(*first);
    let slope: f64 = d / (slice.len() - 1) as f64;
    let d: f64 = Into::<f64>::into(*target) - Into::<f64>::into(*first);
    let estimate_index = (d / slope) as usize;
    let estimate = slice.get(estimate_index)?;

    Some(match target.cmp(estimate) {
        Ordering::Less => linear_search(target, &slice[..estimate_index])?,
        Ordering::Equal => estimate_index,
        Ordering::Greater => {
            estimate_index + 1 + linear_search(target, &slice[estimate_index + 1..])?
        }
    })
}
```

# 其他

既然都能当成一元一次函数来搜索，那不妨对这个数组进行一个映射，变成求极小值问题🤔

```js
>> var array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
>> var target = 5;
>> array.map(x => x - target).map(x => x * x);
<- Array(10) [ 16, 9, 4, 1, 0, 1, 4, 9, 16, 25 ]
```

<div id="graph-find-array-minimum">
  <script src="./graph-random-array.js" defer></script>
  <script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js"></script>
  <div class="chart-container" style="position: relative; height: 400px; overflow: hidden;"></div>
</div>

也可以使用 `find_by_key(|x| (x-target)^2)` 来搜索，避免需要对整个数组进行处理。

这样就变成了一个寻求极小值`0`了，我们可以用到一些最优化的方法来搜索。

# 测试

```sh
$ cargo test
... ok
41 test passed
```

看到这么多 ok 还是很开心的 😆

但是测试代码还是有些问题存在的，比如测试代码并没有对并行或者二分搜索的数组进行去掉重复的元素，重复代码过多等问题。

## Benchmark

浅浅的跑一下分 🏃（仅供娱乐使用）

```sh
$ cargo bench
test tests::benchs::binary::search_1k             ... bench:          13.55 ns/iter (+/- 0.22)
test tests::benchs::binary::search_1m             ... bench:          25.82 ns/iter (+/- 0.33)
test tests::benchs::binary::search_1m_last        ... bench:          22.49 ns/iter (+/- 0.44)
test tests::benchs::linear_search::search_1k      ... bench:          40.45 ns/iter (+/- 1.42)
test tests::benchs::linear_search::search_1m      ... bench:          50.09 ns/iter (+/- 0.78)
test tests::benchs::linear_search::search_1m_last ... bench:           4.27 ns/iter (+/- 0.19)
test tests::benchs::parallel::search_1k           ... bench:      13,421.86 ns/iter (+/- 2,740.09)
test tests::benchs::parallel::search_1m           ... bench:      29,639.51 ns/iter (+/- 2,154.10)
test tests::benchs::parallel::search_1m_last      ... bench:     122,854.11 ns/iter (+/- 11,265.01)
test tests::benchs::sequential::search_1k         ... bench:         423.82 ns/iter (+/- 8.26)
test tests::benchs::sequential::search_1m         ... bench:       5,532.53 ns/iter (+/- 50.02)
test tests::benchs::sequential::search_1m_last    ... bench:     303,801.72 ns/iter (+/- 4,762.71)
test tests::benchs::std_binary::search_1k         ... bench:           9.33 ns/iter (+/- 0.18)
test tests::benchs::std_binary::search_1m         ... bench:          21.04 ns/iter (+/- 0.30)
test tests::benchs::std_binary::search_1m_last    ... bench:          23.08 ns/iter (+/- 0.33)
```

benchmark 代码的实现是有缺陷的，仅能图一乐呵。
比如，测试跑分的数据是随机生成的，每次跑分的成绩都不会相同，正确的做法应该使用一些固定的数组来作为测试，并且跑分的数据太少，不能总体反应其性能。

但也可以大致看到 `std_binary` 标准库的实现是一如既往的优秀 🤣，递归写的也还算可以。
而一元一次函数搜索本次表现的还可以，但实际上表现的并不是特别稳定，而并行和串行并不能被这个测试代码很好的衡量 😢

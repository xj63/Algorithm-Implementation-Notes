+++
title = "排序 | Sort"
date = "2024-09-17"

[taxonomies]
tags = ["array", "sort", "test"]
+++

排序算法（Sorting Algorithm）是一种将一组特定的数据按某种顺序进行排列的算法。

冒泡排序 | 选择排序 | 插入排序 | 归并排序 | 基数排序

<!-- more -->

# 过度设计导致混乱

对一种排序算法 `struct MergeSort`，为其实现排序方法 `impl Sort for MergeSort`。

```rust
pub trait Sort<T, const N: usize> {
    fn sort_method<F>(array: [T; N], is_less: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T, &T) -> bool;

    fn sort(array: [T; N]) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        T: Ord,
    {
        Self::sort_method(array, T::lt)
    }

    fn sort_by<F>(array: [T; N], mut compare: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        Self::sort_method(array, move |a, b| compare(a, b) == Ordering::Less)
    }

    fn sort_by_key<F, K>(array: [T; N], mut key: F) -> Sorted<T, N, impl FnMut(&T, &T) -> bool>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        Self::sort_method(array, move |a, b| key(a) < key(b))
    }
}
```

1. 为了标记数组已经被排好序了，排序函数返回一个被标记为 `Sorted` 的数组。
2. 为了给编译器提供更多的优化信息，排序算法只接受一个长度为 `N` 的数组。
3. 为了可以按照用户想要的顺序排序，排序函数接受一个 `is_less` 函数。

- 对于 `1` 标记数组是有序的没什么问题，已经有序的可以任意的读取，比如使用二分搜索，
  但如果想要修改，必须从 `Sorted` 中 `take` 出来，此时数组就不再被标记为有序的。
  `Sorted` 承诺：包含的数组是永远有序的。

- 对于 `2` 使用泛型来确保数组的长度在编译时确定，提供更多的优化信息，但是丧失了通用性，
  比如想排序一个 `[i32; 8]` 的数组很容易，但是大多时候都是不知道数组的长度的，可能更经常遇到 `&[i32]`。

- 对于 `3` 使用一个函数来确定两个元素之间的大小关系，这也很正常，
  但是和 `Sorted` 有些冲突，`Sorted` 还需要持有一个比较两个元素大小的函数来说明是按照这种顺序排列的。

然后就会发现 `Sorted` 似乎有些没有必要，只能为他实现一些有限的只读方法（如二分搜索），并不是很有用。
只需要程序员确保数组是有序的就可以了，而不是通过类型。同时，这种设计导致了混乱，代码的实现也变得有些复杂和丑陋。

# 排序算法的实现

最终决定使用 c 语言实现排序算法，并尽量追求易读。

## 冒泡排序 | Bubble Sort

```c
static inline void bubble_pass(unsigned len, int array[len]) {
  for (unsigned i = 1; i < len; i++)
    if (array[i - 1] > array[i])
      SWAP(array[i - 1], array[i]);
}

void bubble_sort(unsigned len, int array[len]) {
  for (unsigned i = len; i > 1; i--)
    bubble_pass(i, array);
}
```

`SWAP(x, y)` 是一个宏函数，它的作用是交换两个变量的值。

```c
#define SWAP(x, y)  { int tmp = x; x = y; y = tmp; }
```

冒泡排序就像气泡从水中冒出来，每次冒泡都会把最大的元素顶到数组末尾，称之为一趟 `bubble_pass`，
而想要冒泡排序完所有的元素，只需要 `len` 次 `bubble_pass` 就可以了，
而每次冒泡都确保了最后一个元素一定是最大的，因此下一次冒泡只需要对 `len - 1` 长度的数组进行排序。

也可以写成漂亮的递归形式。

```c
void bubble_sort(unsigned len, int array[len]) {
  if (len <= 1) return;
  bubble_pass(len, array);
  bubble_sort(len - 1, array);
}
```

## 插入排序 | Insertion Sort

```c
// user should ensure array is sorted. [1, 2, 3, 5, ..., N]
static inline int insert_by_ord(unsigned len, int array[len], int element) {
  if (len == 0) return element;

  int last = array[len - 1];
  if (element >= last)
    // element is greater than or equal array max
    return element;

  // sliding window 2
  for (int i = len - 2; i >= 0; i--) {
    int wave = array[i];

    if (element >= wave) {
      // element meet the first less than or equal self
      array[i + 1] = element;
      return last;
    }

    array[i + 1] = wave;
  }

  // element is less than array min
  array[0] = element;
  return last;
}

void insertion_sort(unsigned len, int array[len]) {
  for (unsigned i = 0; i < len; i++) {
    int tmp = insert_by_ord(i, array, array[i]);
    array[i] = tmp;
  }
}
```

将输入数组分为两个部分，前面部分是已经排序的，后面部分是未排序的。
对于第一次执行，前面部分为空 `[i32; 0]`，后面部分为原数组。
对于每次插入操作 `insert_by_ord` 会将 `element` 插入到数组中合适的位置，并返回从数组中挤出来的元素。
将后面的数组所有元素都插入到前面的数组中 `insertion_sort`

## 选择排序 | Selection Sort

```c
typedef struct MinMaxIndex {
  unsigned min;
  unsigned max;
} MinMaxIndex;

static inline MinMaxIndex select_maxmin_index(unsigned len, int array[len]) {
  MinMaxIndex minmax = {0, 0};

  for (unsigned i = 1; i < len; i++) {
    if (array[i] < array[minmax.min])
      minmax.min = i;
    else if (array[i] > array[minmax.max])
      minmax.max = i;
  }

  return minmax;
}

void selection_sort(unsigned len, int array[len]) {
  if (len < 2) return;

  MinMaxIndex minmax = select_maxmin_index(len, array);

  if (minmax.max == 0) {
    SWAP(array[minmax.min], array[0]);
    SWAP(array[minmax.min], array[len - 1]);
  } else {
    SWAP(array[minmax.min], array[0]);
    SWAP(array[minmax.max], array[len - 1]);
  }

  selection_sort(len - 2, &array[1]);
}
```

选择排序通过 `select_maxmin_index` 遍历数组找到最大值和最小值的下标，
然后将最小的元素和第一个元素交换，最大的元素和最后一个元素交换，
然后对于除了第一个和最后一个元素外的剩余元素执行 `selection_sort`。

可能会对交换最大最小元素和首尾元素的代码感到疑惑，其实代码有三版 😭

```c
/// Error: Read the modified area
/// [4, 3, 3, 1] --error-> [4, 3, 3, 1]
SWAP(array[minmax.min], array[0]);
SWAP(array[minmax.max], array[len - 1]);
```

```c
/// Error: Double write.
/// [3, 1, 2] --error-> [1, 3, 3]
// copy
int max = array[minmax.max];
int min = array[minmax.min];
int head = array[0];
int tail = array[len - 1];
// write
array[minmax.min] = head;
array[minmax.max] = tail;
array[len - 1] = max;
array[0] = min;
```

```c
if (minmax.max == 0) {
  SWAP(array[minmax.min], array[0]);
  SWAP(array[minmax.min], array[len - 1]);
} else {
  SWAP(array[minmax.min], array[0]);
  SWAP(array[minmax.max], array[len - 1]);
}
```

这些错误十分隐蔽，只有通过合适的测试才能发现它们。

## 归并排序 | Merge Sort

```c
/// Merge two sorted array into one sorted array
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// //                     ^split        ^len
/// merge_two_sorted_array(8, array, 3);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void merge_two_sorted_array(unsigned len, int array[len], unsigned split);
```

`merge_two_sorted_array` 将两个有序数组合并成一个有序数组。

```c
void merge_sort_rec(unsigned len, int array[len]) {
  if (len <= 1) return;

  unsigned half = len / 2;
  merge_sort_rec(half, array);
  merge_sort_rec(len - half, &array[half]);

  merge_two_sorted_array(len, array, half);
}
```

`merge_sort_rec` 递归地将数组划分成更小的数组，直到数组长度小于等于1，然后合并两个有序数组。

对于自底向上，将数组划分为一系列的有序的块(n)，对于每个两个相邻的块进行合并为一个块(n/2)，
每次合并其块总数变为一半，块的大小变为两倍，然后再对相邻块合并，直到块的大小超过了数组长度。

```c
void merge_adjacent_blocks(unsigned len, int array[len], unsigned block_size) {
  if (block_size >= len)
    // Only have one block and have sorted.
    return;

  unsigned blocks = len / block_size; // blocks >= 1
  for (unsigned i = 0; i < blocks / 2; i++)
    merge_two_sorted_array(block_size * 2, &array[i * 2 * block_size], block_size);

  if (blocks % 2 == 1 && len > block_size * blocks) {
    // The last block is not full and remain the second last block not merge.
    unsigned lave = len - block_size * blocks;
    merge_two_sorted_array(block_size + lave, &array[len - lave - block_size], block_size);
  }
}

void merge_sort_adjacent_blocks(unsigned len, int array[len]) {
  for (unsigned block_size = 1; block_size <= len; block_size *= 2)
    merge_adjacent_blocks(len, array, block_size);
}
```

然后我们就可以将数组划分为多个块，并行的进行排序。
使用 `openmp` 对多个块并行排序，然后再合并排好序的数组。

```c
// return block_size
unsigned parallel_sort_blocks(unsigned len, int array[len]) {
  const unsigned DEFAULT_BLOCK_SIZE = 128;
  const unsigned MAX_BLOCKS = 64;
  unsigned block_size = len / DEFAULT_BLOCK_SIZE > MAX_BLOCKS
                            ? len / MAX_BLOCKS
                            : DEFAULT_BLOCK_SIZE;
  unsigned blocks = len / block_size;

#pragma omp parallel for
  for (unsigned i = 0; i < blocks; i++)
    merge_sort_rec(block_size, &array[i * block_size]);

  if (len > block_size * blocks)
    // the last block is not full.
    merge_sort_rec(len - (blocks * block_size), &array[blocks * block_size]);

  return block_size;
}

void merge_sort_parallel(unsigned len, int array[len]) {
  unsigned block_size = parallel_sort_blocks(len, array);
  for (; block_size <= len; block_size *= 2)
    merge_adjacent_blocks(len, array, block_size);
}
```

## 基数排序 | Radix LSD Sort

```c
typedef struct LinkNode {
  unsigned data;
  struct LinkNode *next;
} LinkNode;

typedef struct LinkList {
  LinkNode *head;
  LinkNode *tail;
} LinkList;
```

定义链表，使用 `VLA` 可变长数组在栈上创建一段缓冲区 `LinkNode node_buf[len]`

基数排序将一个数字分为 `num_of_keys` 个子数字，每个子数字的范围是 `[0, base)`。
根据最后一个子数字的大小，将元素分到 `base` 个桶中，然后再将桶合并。
接下来根据第二个子数字的大小，分配然后合并，如此往复，最终将整个数组排序。

```c
/// Radix LSD Sort
///
/// default base is 256, number of keys is 4
/// radix_lsd_sort_with(len, array[len], 256, 4);
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// radix_sort(8, array);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void radix_lsd_sort(unsigned len, int array[len]) {
  const unsigned BIAS = UINT_MAX / 2 + 1;
  for (unsigned i = 0; i < len; i++)
    array[i] = (unsigned)array[i] + BIAS;

  radix_lsd_sort_with(len, (unsigned *)array, 256, 4);

  for (unsigned i = 0; i < len; i++)
    array[i] = (int)(array[i] - BIAS);
}
```

为了能对有符号数据进行基数排序，我们需要将数据转换为无符号数并保留大小关系，排完序后再转换回来。
然后调用 `radix_lsd_sort_with(len, array, 256, 4)` 以基数 256 排序，并将数字分为 4 个子数字。
对于编译器能否将常量基数 256 优化成右移，看造化吧 :)

```c
/// Radix LSD Sort with base and number of keys
///
/// Split the element into `num_of_keys` keys no greater than `base`
///
/// # Warning
///
/// The element of `array` must be no greater than `num_of_keys * base`
static inline void radix_lsd_sort_with(unsigned len, unsigned array[len],
                                       unsigned base, unsigned num_of_keys) {
  LinkNode node_buf[len];
  LinkList list = array2linklist(len, array, node_buf);

  for (unsigned i = 0, offset = 1; i < num_of_keys; i += 1, offset *= base)
    list = radix_split_and_merge(list, offset, base);

  linklist2array(len, list, array);
}
```

先将数组转换为链表，然后对链表进行基数排序，最后再将链表转换为数组。
对链表进行基数排序，只需要对链表中的元素 `(e / offset) % base` 然后桶装然后合并，
如此重复 `num_of_keys` 次（并更新 `offset`）就获得一个有序的链表。

```c
static inline LinkList radix_split_and_merge(LinkList list, unsigned offset,
                                             unsigned base) {
  LinkList bucket[base];
  for (unsigned i = 0; i < base; i++)
    bucket[i] = (LinkList){.head = NULL, .tail = NULL};

  LinkNode *iter = list.head;
  while (iter != NULL) {
    unsigned index = (iter->data / offset) % base;
    LinkNode *next = linklist_push_one(&bucket[index], iter);
    iter = next;
  }

  LinkList result = bucket[0];
  for (unsigned i = 1; i < base; i++)
    result = linklist_append(result, bucket[i]);
  return result;
}
```

迭代遍历链表，将链表中的元素 `(e / offset) % base` 然后插入桶 `bucket[base]` 中，然后合并所有桶。

# BenchMark

本次设计仍有问题，不具有参考价值。

```console
test csort::tests::bench::bubble::gaussian_with_noise                ... bench:     340,889.20 ns/iter (+/- 130,392.50)
test csort::tests::bench::bubble::high_sample_sin_with_noise         ... bench:     339,712.45 ns/iter (+/- 12,469.97)
test csort::tests::bench::bubble::low_sample_sin_with_noise          ... bench:     340,981.80 ns/iter (+/- 110,796.67)
test csort::tests::bench::bubble::random                             ... bench:  34,957,551.50 ns/iter (+/- 3,410,550.90)
test csort::tests::bench::bubble::stroll                             ... bench:  34,862,127.00 ns/iter (+/- 3,108,527.50)
test csort::tests::bench::bubble::trend_increasing                   ... bench:     341,875.30 ns/iter (+/- 125,752.78)
test csort::tests::bench::cstd_qsort::gaussian_with_noise            ... bench:      14,948.66 ns/iter (+/- 9,671.89)
test csort::tests::bench::cstd_qsort::high_sample_sin_with_noise     ... bench:      15,045.78 ns/iter (+/- 9,829.04)
test csort::tests::bench::cstd_qsort::low_sample_sin_with_noise      ... bench:      15,038.26 ns/iter (+/- 3,390.93)
test csort::tests::bench::cstd_qsort::random                         ... bench:     185,289.35 ns/iter (+/- 90,925.77)
test csort::tests::bench::cstd_qsort::stroll                         ... bench:     186,603.78 ns/iter (+/- 54,371.42)
test csort::tests::bench::cstd_qsort::trend_increasing               ... bench:      14,965.57 ns/iter (+/- 990.55)
test csort::tests::bench::insertion::gaussian_with_noise             ... bench:         474.50 ns/iter (+/- 52.69)
test csort::tests::bench::insertion::high_sample_sin_with_noise      ... bench:         474.28 ns/iter (+/- 125.33)
test csort::tests::bench::insertion::low_sample_sin_with_noise       ... bench:         468.62 ns/iter (+/- 178.76)
test csort::tests::bench::insertion::random                          ... bench:       4,677.77 ns/iter (+/- 1,472.75)
test csort::tests::bench::insertion::stroll                          ... bench:       4,702.79 ns/iter (+/- 3,031.50)
test csort::tests::bench::insertion::trend_increasing                ... bench:         474.13 ns/iter (+/- 216.46)
test csort::tests::bench::merge::gaussian_with_noise                 ... bench:       8,029.80 ns/iter (+/- 2,176.44)
test csort::tests::bench::merge::high_sample_sin_with_noise          ... bench:       7,707.73 ns/iter (+/- 813.41)
test csort::tests::bench::merge::low_sample_sin_with_noise           ... bench:       7,769.37 ns/iter (+/- 59.58)
test csort::tests::bench::merge::random                              ... bench:      84,954.48 ns/iter (+/- 1,245.39)
test csort::tests::bench::merge::stroll                              ... bench:      85,204.15 ns/iter (+/- 2,103.85)
test csort::tests::bench::merge::trend_increasing                    ... bench:       7,769.06 ns/iter (+/- 66.77)
test csort::tests::bench::merge_parallel::gaussian_with_noise        ... bench:       5,455.78 ns/iter (+/- 1,242.05)
test csort::tests::bench::merge_parallel::high_sample_sin_with_noise ... bench:       5,548.66 ns/iter (+/- 1,807.09)
test csort::tests::bench::merge_parallel::low_sample_sin_with_noise  ... bench:       5,476.28 ns/iter (+/- 494.94)
test csort::tests::bench::merge_parallel::random                     ... bench:      36,199.32 ns/iter (+/- 5,999.14)
test csort::tests::bench::merge_parallel::stroll                     ... bench:      36,338.08 ns/iter (+/- 11,186.60)
test csort::tests::bench::merge_parallel::trend_increasing           ... bench:       5,486.77 ns/iter (+/- 408.39)
test csort::tests::bench::radix_lsd::gaussian_with_noise             ... bench:       9,347.28 ns/iter (+/- 159.57)
test csort::tests::bench::radix_lsd::high_sample_sin_with_noise      ... bench:       9,176.39 ns/iter (+/- 197.14)
test csort::tests::bench::radix_lsd::low_sample_sin_with_noise       ... bench:       8,921.27 ns/iter (+/- 189.30)
test csort::tests::bench::radix_lsd::random                          ... bench:      73,262.34 ns/iter (+/- 536.66)
test csort::tests::bench::radix_lsd::stroll                          ... bench:      68,302.56 ns/iter (+/- 778.09)
test csort::tests::bench::radix_lsd::trend_increasing                ... bench:       8,745.26 ns/iter (+/- 206.40)
test csort::tests::bench::rust_stable::gaussian_with_noise           ... bench:         237.93 ns/iter (+/- 8.98)
test csort::tests::bench::rust_stable::high_sample_sin_with_noise    ... bench:         235.83 ns/iter (+/- 186.60)
test csort::tests::bench::rust_stable::low_sample_sin_with_noise     ... bench:         236.56 ns/iter (+/- 5.78)
test csort::tests::bench::rust_stable::random                        ... bench:       2,346.31 ns/iter (+/- 37.19)
test csort::tests::bench::rust_stable::stroll                        ... bench:       2,360.04 ns/iter (+/- 43.39)
test csort::tests::bench::rust_stable::trend_increasing              ... bench:         235.54 ns/iter (+/- 3.81)
test csort::tests::bench::selection::gaussian_with_noise             ... bench:     372,126.70 ns/iter (+/- 4,828.52)
test csort::tests::bench::selection::high_sample_sin_with_noise      ... bench:     370,465.57 ns/iter (+/- 2,651.91)
test csort::tests::bench::selection::low_sample_sin_with_noise       ... bench:     370,717.30 ns/iter (+/- 3,108.06)
test csort::tests::bench::selection::random                          ... bench:  37,867,238.80 ns/iter (+/- 393,008.45)
test csort::tests::bench::selection::stroll                          ... bench:  37,757,740.20 ns/iter (+/- 315,227.42)
test csort::tests::bench::selection::trend_increasing                ... bench:     380,573.42 ns/iter (+/- 6,010.24)
test merge_two_sorted::tests::bench::array1k                         ... bench:         238.10 ns/iter (+/- 4.32)
```

+++
title = "排序 | Sort"
date = "2024-09-17"

[taxonomies]
tags = ["array", "sort", "bench", "test"]
+++

排序算法（Sorting Algorithm）是一种将一组特定的数据按某种顺序进行排列的算法。

冒泡排序 | 选择排序 | 插入排序 | 归并排序

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

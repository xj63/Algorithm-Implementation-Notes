#define SWAP(x, y)                                                             \
  {                                                                            \
    int tmp = x;                                                               \
    x = y;                                                                     \
    y = tmp;                                                                   \
  }

/// Merge two sorted array into one sorted array
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// //                     ^split        ^len
/// merge_two_sorted_array(8, array, 3);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void merge_two_sorted_array(unsigned len, int array[len], unsigned split);

/// Bubble sort
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// bubble_sort(8, array);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void bubble_sort(unsigned len, int array[len]);

/// Selection sort
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// selection_sort(8, array);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void selection_sort(unsigned len, int array[len]);

/// Insertion sort
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// insertion_sort(8, array);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void insertion_sort(unsigned len, int array[len]);

/// Merge sort
///
/// # Example
///
/// int array[] = {1, 3, 5, 0, 3, 4, 4, 8};
/// merge_sort(8, array);
/// // array = {0, 1, 3, 3, 4, 4, 5, 8}
void merge_sort(unsigned len, int array[len]);

/// Merge sort parallel.
void merge_sort_parallel(unsigned len, int array[len]);

/// c std qsort
void std_qsort(unsigned len, int array[len]);

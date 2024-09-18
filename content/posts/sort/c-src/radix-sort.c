#include "sort.h"
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct LinkNode {
  unsigned data;
  struct LinkNode *next;
} LinkNode;

typedef struct LinkList {
  LinkNode *head;
  LinkNode *tail;
} LinkList;

static inline void linklist_display(LinkList list) {
  for (LinkNode *iter = list.head; iter != NULL; iter = iter->next)
    printf("%u ", iter->data);
  printf("\n");
}

static inline LinkList linklist_append(LinkList left, LinkList right) {
  if (left.head == NULL)
    return right;
  if (right.head == NULL)
    return left;

  left.tail->next = right.head;
  left.tail = right.tail;
  return left;
}

static inline LinkNode *linklist_push_one(LinkList *list, LinkNode *element) {
  if (list->tail == NULL) {
    list->head = element;
    list->tail = element;
  } else {
    list->tail->next = element;
    list->tail = element;
  }

  LinkNode *next = element->next;
  element->next = NULL;
  return next;
}

static inline LinkList array2linklist(unsigned len, const unsigned array[len],
                                      LinkNode buf[len]) {
  if (len == 0)
    return (LinkList){.head = NULL, .tail = NULL};

  for (unsigned i = 0; i < len; i++) {
    buf[i].data = array[i];
    buf[i].next = &buf[i + 1];
  }
  buf[len - 1].next = NULL;

  return (LinkList){.head = &buf[0], .tail = &buf[len - 1]};
}

/// user should ensure list len equal len.
static inline void linklist2array(unsigned len, const LinkList list,
                                  unsigned array[len]) {
  unsigned i = 0;
  for (LinkNode *iter = list.head; iter != NULL; iter = iter->next)
    array[i++] = iter->data;
}

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

/* #define TEST */
#ifdef TEST

#include "test-utils.c"

void test_bias() {
  int array[] = {-1, 8, 19, -3, 996, INT_MIN, INT_MAX, 0, 1, 2, 3, 4, 5, 6, 7};
  radix_lsd_sort(ARRAY_LEN(array), array);
  DISPLAY_ARRAY(array);
}

int main(int argc, char *argv[]) {
  test_bias();
  return EXIT_SUCCESS;
}

#endif /* ifdef TEST */

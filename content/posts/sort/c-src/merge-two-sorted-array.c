#include <stdbool.h>
#include <string.h>

struct Stack {
  int *data;
  unsigned len;
};

#define NEW_STACK(IDENT, LEN)                                                  \
  int __stack_##IDENT##_buf_size_##LEN[LEN]; /* stack cap is len */            \
  struct Stack IDENT = {.data = __stack_##IDENT##_buf_size_##LEN, .len = 0};

// user should ensure stack->len < stack buf size
static inline void stack_push(struct Stack *stack, int value) {
  stack->data[stack->len++] = value;
}

// user should ensure stack->len > 0
static inline int stack_pop(struct Stack *stack) {
  return stack->data[--stack->len];
}

// user should ensure stack->len < stack buf size
/* Not use
static void stack_extend(struct Stack *stack, const int *slice, unsigned len) {
  memcpy(stack->data + stack->len, slice, len * sizeof(int));
  stack->len += len;
}
*/

// user should ensure stack->len > 0
// move data from stack->data[stack->len - len..stack->len] to slice
static void stack_drain(struct Stack *stack, int *slice, unsigned len) {
  memcpy(slice, &stack->data[stack->len - len], len * sizeof(int));
  stack->len -= len;
}

void merge_two_sorted_array(unsigned len, int array[len], unsigned split) {
  if (len <= 1 || split >= len || split == 0)
    return;

  NEW_STACK(stack, len);

  int *left = array;
  int *divide = &array[split];
  int *right = divide;
  int *end = &array[len];

  /* while (left < divide && right < end) { */
  while (true) {
    if (*left <= *right) {
      stack_push(&stack, *left);
      left += 1;

      if (left >= divide)
        break;
    } else {
      stack_push(&stack, *right);
      right += 1;

      if (right >= end) {
        /* Move the remaining elements on the left to the stack.
         * But this will cause unnecessary secondary copying.
         * Just move the elements on the left to the back of the array.
         *
         * if (left < divide)
         * // There is still something left on the left side.
         * // The reason for exiting is that the right side has been traversed.
         * stack_extend(&stack, left, divide - left);
         */
        unsigned lave = divide - left;
        memcpy(&array[len - lave], left, lave * sizeof(int));

        break;
      }
    }
  }

  stack_drain(&stack, array, stack.len);
}

/* #define TEST */
#ifdef TEST

#include <stdio.h>

void test_stack() {
  NEW_STACK(stack, 5);

  stack_push(&stack, 1);
  stack_push(&stack, 2);
  stack_push(&stack, 3);
  stack_push(&stack, 4);
  stack_push(&stack, 5);

  int array[5];
  stack_drain(&stack, array, 5);
  for (int i = 0; i < 5; i++) {
    printf("%d ", array[i]);
  }
  printf("\n");
}

int main() {
  test_stack();
  int array[] = {};

  merge_two_sorted_array(0, array, 0);

  for (int i = 0; i < 1; i++) {
    printf("%d ", array[i]);
  }
  printf("\n");
}

#endif /* ifdef TEST */

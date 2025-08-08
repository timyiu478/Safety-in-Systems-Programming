# Memory Safety

Today’s lecture will focus on memory errors that often arise in programs. We will look at how these errors motivate Rust’s ownership model, and explain how Rust prevents them.

This exercise was developed by Will Crichton for CS 242. Thank you, Will, for letting us borrow this material!

Please spend just 10 minutes reviewing the following C implementation of a vector. There are at least **7** bugs. You don’t need to catch ‘em all, but try to spot as many as you can.

Please write down the bugs you find.

# Bugs

The found bugs are written as inline comments with prefex `B`.

```rust
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

// There are at least 7 bugs relating to memory on this snippet.
// Find them all!

// Vec is short for "vector", a common term for a resizable array.
// For simplicity, our vector type can only hold ints.
typedef struct {
  int* data;     // Pointer to our array on the heap
  int  length;   // How many elements are in our array
  int  capacity; // How many elements our array can hold
} Vec;

Vec* vec_new() {
  Vec vec;
  vec.data = NULL;
  vec.length = 0;
  vec.capacity = 0; // B1. if vec.capacity is 0, then this vec can't double it capacity by multiplying 2 in vec_push() function
  return &vec; // B2. Dangling Pointer: &vec is address of vec but vec is a local variable of vec_new()
}

void vec_push(Vec* vec, int n) {
  if (vec->length == vec->capacity) {
    int new_capacity = vec->capacity * 2;
    int* new_data = (int*) malloc(new_capacity); // B6. malloc(0) becase of B1
    assert(new_data != NULL);

    for (int i = 0; i < vec->length; ++i) {
      new_data[i] = vec->data[i];
    }

    vec->data = new_data; // B3. vec->data point to larger heap space without freeing the old heap first
    vec->capacity = new_capacity;
  }

  vec->data[vec->length] = n;
  ++vec->length;
}

void vec_free(Vec* vec) {
  free(vec); 
  free(vec->data); // B5. if vec is freed first, then we will lose the vec->data pointer.
}

void main() {
  Vec* vec = vec_new();
  vec_push(vec, 107);

  int* n = &vec->data[0];
  vec_push(vec, 110);
  printf("%d\n", *n); // B7. iterator invalidation - because the above vec_push() switched the heap space for vec->data

  free(vec->data);
  vec_free(vec); // B4. double free of vec->data
}
```

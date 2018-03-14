#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct A A;

typedef struct B B;

typedef struct {
  B *members;
  uintptr_t count;
} List_B;

typedef struct {
  A *members;
  uintptr_t count;
} List_A;

void bar(List_B b);

void foo(List_A a);

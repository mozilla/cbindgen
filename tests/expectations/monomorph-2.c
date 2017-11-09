#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct A;
typedef struct A A;

struct B;
typedef struct B B;

typedef struct {
  B *members;
  size_t count;
} List_B;

typedef struct {
  A *members;
  size_t count;
} List_A;

void bar(List_B b);

void foo(List_A a);

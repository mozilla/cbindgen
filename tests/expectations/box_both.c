#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_____i32 NotReprC_____i32;

typedef struct NotReprC_____i32 Foo;

typedef struct MyStruct {
  int32_t *number;
} MyStruct;

void root(const Foo *a, const struct MyStruct *with_box);

void drop_box(int32_t *x);

void drop_box_opt(int32_t *x);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC______i32 NotReprC______i32;

typedef NotReprC______i32 Foo;

typedef struct {
  const int32_t *number;
} MyStruct;

void root(const Foo *a, const MyStruct *with_maybe_uninit);

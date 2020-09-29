#ifdef __cplusplus
template <typename T>
using Box = T*;
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_Box_i32 NotReprC_Box_i32;

typedef NotReprC_Box_i32 Foo;

typedef struct MyStruct {
  int32_t *number;
} MyStruct;

void drop_box(int32_t *x);

void root(const Foo *a, const MyStruct *with_box);

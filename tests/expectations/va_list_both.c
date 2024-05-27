#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*VaListFnPtr)(...);

typedef int32_t (*VaListFnPtr2)(void);

typedef struct Interface_______i32_______va_list {
  int32_t (*fn1)(...);
} Interface_______i32_______va_list;

typedef struct Interface_______i32 {
  int32_t (*fn1)(void);
} Interface_______i32;

int32_t va_list_test(...);

int32_t va_list_test2(...);

void va_list_fn_ptrs(int32_t (*fn1)(...),
                     int32_t (*fn2)(void),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     struct Interface_______i32_______va_list fn5,
                     struct Interface_______i32 fn6);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*VaListFnPtr)(va_list);

typedef int32_t (*VaListFnPtr2)(va_list);

typedef struct {
  int32_t (*fn1)(va_list);
} Interface_______i32_______va_list;

int32_t va_list_test(va_list ap);

int32_t va_list_test2(va_list ap);

void va_list_fn_ptrs(int32_t (*fn1)(va_list),
                     int32_t (*fn2)(va_list),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface_______i32_______va_list fn5,
                     Interface_______i32_______va_list fn6);

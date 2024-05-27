#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int32_t (*VaListFnPtr)(int32_t count, va_list);

typedef int32_t (*VaListFnPtr2)(int32_t count);

typedef struct Interface_______i32_______i32_______va_list {
  int32_t (*fn1)(int32_t count, va_list);
} Interface_______i32_______i32_______va_list;

typedef struct Interface_______i32_______i32 {
  int32_t (*fn1)(int32_t count);
} Interface_______i32_______i32;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int32_t va_list_test(int32_t count, va_list ap);

int32_t va_list_test2(int32_t count, va_list ap);

void va_list_fn_ptrs(int32_t (*fn1)(int32_t count, va_list),
                     int32_t (*fn2)(int32_t count),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     struct Interface_______i32_______i32_______va_list fn5,
                     struct Interface_______i32_______i32 fn6);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

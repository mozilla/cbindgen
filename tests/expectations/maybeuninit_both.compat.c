#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC______i32 NotReprC______i32;

typedef struct NotReprC______i32 Foo;

typedef struct MyStruct {
  const int32_t *number;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const struct MyStruct *with_maybe_uninit);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

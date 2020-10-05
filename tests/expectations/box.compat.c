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

typedef struct {
  int32_t *number;
} MyStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const MyStruct *with_box);

void drop_box(int32_t *x);

void drop_box_opt(int32_t *x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

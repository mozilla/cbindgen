#if 0
''' '
#endif

#ifdef __cplusplus
template <typename T>
using ManuallyDrop = T;
#endif

#if 0
' '''
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct MyStruct;

struct NotReprC_Point;

struct Point {
  int32_t x;
  int32_t y;
};

typedef struct NotReprC_Point Foo;

struct MyStruct {
  struct Point point;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const Foo *a, const struct MyStruct *with_manual_drop);

void take(struct Point with_manual_drop);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

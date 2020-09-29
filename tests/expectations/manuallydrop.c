#ifdef __cplusplus
template <typename T>
using ManuallyDrop = T;
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct NotReprC_ManuallyDrop_Point NotReprC_ManuallyDrop_Point;

typedef NotReprC_ManuallyDrop_Point Foo;

typedef struct {
  int32_t x;
  int32_t y;
} Point;

typedef struct {
  Point point;
} MyStruct;

void root(const Foo *a, const MyStruct *with_manual_drop);

void take(Point with_manual_drop);

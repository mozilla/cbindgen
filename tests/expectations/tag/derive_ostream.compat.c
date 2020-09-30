#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum C
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  X = 2,
  Y,
};
#ifndef __cplusplus
typedef uint32_t C;
#endif // __cplusplus

struct A {
  int32_t _0;
};

struct B {
  int32_t x;
  float y;
};

struct D {
  uint8_t List;
  uintptr_t Of;
  struct B Things;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a, struct B b, C c, struct D d);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

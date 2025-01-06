#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct B;

struct A {
  struct B *buf;
  uintptr_t len;
};

struct B {
  int32_t something;
  struct A nested;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct B *foo);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

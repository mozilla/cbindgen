#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct B;

typedef struct A {
  struct B *buf;
  uintptr_t len;
} A;

typedef struct B {
  int32_t something;
  struct A nested;
} B;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct B *foo);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

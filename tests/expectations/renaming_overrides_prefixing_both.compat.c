#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct StyleA;

typedef struct B {
  int32_t x;
  float y;
} B;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct StyleA *a, struct B b);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

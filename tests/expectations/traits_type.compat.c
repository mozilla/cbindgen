#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uintptr_t dummy;
} Dummy0;

typedef struct {
  uintptr_t dummy;
} Dummy1;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

Dummy0 dummy_Dummy0(Dummy0 self, uintptr_t in_);

int32_t dummy_Dummy1(Dummy1 self);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

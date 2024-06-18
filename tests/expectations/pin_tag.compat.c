#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct PinTest {
  int32_t *pinned_box;
  int32_t *pinned_ref;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(int32_t *s, struct PinTest p);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

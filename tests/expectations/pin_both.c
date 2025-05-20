#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PinTest {
  int32_t *pinned_box;
  int32_t *pinned_ref;
} PinTest;

void root(int32_t *s, struct PinTest p);

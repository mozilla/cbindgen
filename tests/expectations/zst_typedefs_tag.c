#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*C_1Z)(uint8_t);

typedef const void *D_1Z;

struct Wrapper_1Z {
  C_1Z c;
  D_1Z d;
};

void use_typedefs_ptr(const struct Wrapper_1Z *w);

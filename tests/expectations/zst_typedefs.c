#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*C_1Z)(uint8_t);

typedef const void *D_1Z;

typedef struct {
  C_1Z c;
  D_1Z d;
} Wrapper_1Z;

void use_typedefs_ptr(const Wrapper_1Z *w);

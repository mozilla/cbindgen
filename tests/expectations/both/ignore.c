#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct OneFieldIgnored {
  int32_t x;
  int32_t z;
} OneFieldIgnored;

typedef struct AllFieldsIgnored {

} AllFieldsIgnored;

void no_ignore_root(OneFieldIgnored one, AllFieldsIgnored all);

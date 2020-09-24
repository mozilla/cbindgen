#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct OneFieldIgnored {
  int32_t x;
  int32_t z;
};

struct AllFieldsIgnored {

};

void no_ignore_root(struct OneFieldIgnored one, struct AllFieldsIgnored all);

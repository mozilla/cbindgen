/* Package version: 0.1.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo;

typedef struct Foo {
  uint64_t bar;
} Foo;

void doit(const struct Foo*);

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
};
static const PREFIXFoo PREFIXFoo_FOO = { /* .a = */ 42, /* .b = */ 47 };

static const PREFIXFoo PREFIXBAR = { /* .a = */ 42, /* .b = */ 1337 };

extern "C" {

void root(PREFIXFoo x);

} // extern "C"

#include <cstdint>
#include <cstdlib>

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
};

static const PREFIXFoo PREFIXBAR = (PREFIXFoo){ .a = 42, .b = 1337 };

static const PREFIXFoo PREFIXFoo_FOO = (PREFIXFoo){ .a = 42, .b = 47 };

extern "C" {

void root(PREFIXFoo x);

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct PREFIXBar {
  int32_t a;
};

struct PREFIXFoo {
  int32_t a;
  uint32_t b;
  PREFIXBar bar;
};

static const PREFIXFoo PREFIXVAL = { /* .a = */ 42, /* .b = */ 1337, /* .bar = */ { /* .a = */ 323 } };

extern "C" {

void root(PREFIXFoo x);

} // extern "C"

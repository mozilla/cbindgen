#include <cstdint>
#include <cstdlib>

struct Opaque;

struct Foo {
  const Opaque *x;
  Opaque *y;
  void (*z)();
};

union Bar {
  const Opaque *x;
  Opaque *y;
  void (*z)();
};

extern "C" {

void root(const Opaque *a, Opaque *b, Foo c, Bar d);

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct Opaque;

struct Foo {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
};

union Bar {
  const Opaque *x;
  Opaque *y;
  void (*z)(void);
};

extern "C" {

void root(const Opaque *a, Opaque *b, Foo c, Bar d);

} // extern "C"

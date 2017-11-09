#include <cstdint>
#include <cstdlib>

extern "C" {

#if (defined(NOT_DEFINED) || defined(DEFINED))
struct Foo {
  int32_t x;
};
#endif

#if defined(NOT_DEFINED)
struct Bar {
  Foo y;
};
#endif

#if defined(DEFINED)
struct Bar {
  Foo z;
};
#endif

struct Root {
  Bar w;
};

void root(Root a);

} // extern "C"

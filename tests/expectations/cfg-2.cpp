#include <cstdint>
#include <cstdlib>

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

extern "C" {

void root(Root a);

} // extern "C"

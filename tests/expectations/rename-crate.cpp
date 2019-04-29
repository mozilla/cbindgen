#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct RenamedTy {
  uint64_t y;
};

struct Foo {
  int32_t x;
};

extern "C" {

void renamed_func(RenamedTy a);

void root(Foo a);

} // extern "C"

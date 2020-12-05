#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct Foo {
  const int32_t *something;
};

union Bar {
  enum class Tag : uint8_t {
    Min,
    Max,
    Other,
  };

  struct {
    Tag tag;
  };
  struct {
    Tag min_tag;
    Foo<Bar> min;
  };
  struct {
    Tag max_tag;
    Foo<Bar> max;
  };
};

extern "C" {

void root(Bar b);

} // extern "C"

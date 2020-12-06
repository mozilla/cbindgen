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
    union {
      Tag min_tag;
    };
    union {
      Foo<Bar> min;
    };
  };
  struct {
    union {
      Tag max_tag;
    };
    union {
      Foo<Bar> max;
    };
  };
};

extern "C" {

void root(Bar b);

} // extern "C"

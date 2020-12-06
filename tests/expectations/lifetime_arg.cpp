#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  const int32_t *data;
};

struct E {
  enum class Tag {
    V,
    U,
  };

  Tag tag;
  union {
    struct {
      union {
        const uint8_t *u;
      };
    };
  };
};

extern "C" {

void root(A _a, E _e);

} // extern "C"

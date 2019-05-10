#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct A {
  int32_t namespace_;
  float float_;
};

struct B {
  int32_t namespace_;
  float float_;
};

struct C {
  enum class Tag : uint8_t {
    D,
  };

  struct D_Body {
    int32_t namespace_;
    float float_;
  };

  Tag tag;
  union {
    D_Body d;
  };
};

extern "C" {

void root(A a, B b, C c, int32_t namespace_, float float_);

} // extern "C"

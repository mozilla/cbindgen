#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class C : uint32_t {
  X = 2,
  Y,
};

inline std::ostream& operator<<(std::ostream& stream, const C& instance) {
  switch (instance) {
    case C::X: stream << "X"; break;
    case C::Y: stream << "Y"; break;
  }
  return stream;
}

struct A {
  int32_t _0;

  friend std::ostream& operator<<(std::ostream& stream, const A& instance) {
    return stream << "{ " << "_0=" << instance._0 << " }";
  }
};

struct B {
  int32_t x;
  float y;

  friend std::ostream& operator<<(std::ostream& stream, const B& instance) {
    return stream << "{ " << "x=" << instance.x << ", "
                          << "y=" << instance.y << " }";
  }
};

struct D {
  uint8_t List;
  uintptr_t Of;
  B Things;

  friend std::ostream& operator<<(std::ostream& stream, const D& instance) {
    return stream << "{ " << "List=" << instance.List << ", "
                          << "Of=" << instance.Of << ", "
                          << "Things=" << instance.Things << " }";
  }
};

extern "C" {

void root(A a, B b, C c, D d);

} // extern "C"

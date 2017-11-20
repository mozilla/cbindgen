#include <cstdint>
#include <cstdlib>

enum class C : uint32_t {
  X = 2,
  Y,
};

struct A {
  int32_t m0;

  A(int32_t const& aM0)
    : m0(aM0)
  {}

  bool operator<(const A& other) const {
    return m0 < other.m0;
  }
  bool operator<=(const A& other) const {
    return m0 <= other.m0;
  }
};

struct B {
  int32_t x;
  float y;
};

extern "C" {

void root(A x, B y, C z);

} // extern "C"

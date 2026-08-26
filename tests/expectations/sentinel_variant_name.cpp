#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class A : uint8_t {
  A_A1,
  A_A2,
  A_A3,
  /// Must be last for serialization purposes
  A_Terminator,
};

enum class B : uint8_t {
  B_B1,
  B_B2,
  B_B3,
  /// Must be last for serialization purposes
  B_Last,
};

extern "C" {

void root(A a, B b);

}  // extern "C"

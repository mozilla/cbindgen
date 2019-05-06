#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const int32_t C_H = 10;

enum class C_E : uint8_t {
  x = 0,
  y = 1,
};

struct C_A;

struct C_C;

struct C_AwesomeB {
  int32_t x;
  float y;
};

union C_D {
  int32_t x;
  float y;
};

using C_F = C_A;

extern "C" {

extern const int32_t G;

void root(const C_A *a, C_AwesomeB b, C_C c, C_D d, C_E e, C_F f);

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const int8_t FOUR = 4;

enum class E : int8_t {
  A = 1,
  B = -1,
  C = (1 + 2),
  D = FOUR,
  F = 5,
};

enum class E_NoCython : int8_t {
  G = (int8_t)'6',
  H = (int8_t)false,
};

extern "C" {

void root(const E*);

void root_no_cython(const E_NoCython*);

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  const int32_t *data;
};

extern "C" {

void root(A _a);

} // extern "C"

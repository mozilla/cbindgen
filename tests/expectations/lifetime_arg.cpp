#include <cstdarg>
#include <cstdint>
#include <cstdlib>

struct A {
  const int32_t *data;
};

extern "C" {

void root(A _a);

} // extern "C"

#include <cstdint>
#include <cstdlib>

struct Normal {
  int32_t x;
  float y;
};

extern "C" {

extern void bar(Normal a);

extern int32_t foo();

} // extern "C"

#include <cstdint>
#include <cstdlib>

extern "C" {

struct Normal {
  int32_t x;
  float y;
};

extern void bar(Normal a);

extern int32_t foo();

} // extern "C"

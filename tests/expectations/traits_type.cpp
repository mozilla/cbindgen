#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Dummy0 {
  uintptr_t dummy;
};

struct Dummy1 {
  uintptr_t dummy;
};

extern "C" {

Dummy0 dummy_Dummy0(Dummy0 self, uintptr_t in_);

int32_t dummy_Dummy1(Dummy1 self);

}  // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct B;

struct A {
  B *buf;
  uintptr_t len;
};

struct B {
  int32_t something;
  A nested;
};

extern "C" {

void root(const B *foo);

}  // extern "C"

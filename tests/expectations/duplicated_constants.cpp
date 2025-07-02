#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Foo {
  uint32_t field;
};
constexpr static const uint32_t Foo_FIELD_RELATED_CONSTANT = 0;

struct Bar {
  uint32_t field;
};
constexpr static const uint32_t Bar_FIELD_RELATED_CONSTANT = 0;

extern "C" {

void root(Foo a, Bar b);

}  // extern "C"

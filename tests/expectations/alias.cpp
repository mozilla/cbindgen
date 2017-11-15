#include <cstdint>
#include <cstdlib>

enum class Status : uint32_t {
  Ok = 0,
  Err = 1,
};

struct Dep {
  int32_t a;
  float b;
};

template<typename X>
struct Foo {
  X a;
  X b;
  Dep c;
};

typedef Foo<int32_t> IntFoo;

typedef Foo<double> DoubleFoo;

typedef int32_t Unit;

typedef Status SpecialStatus;

extern "C" {

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

} // extern "C"

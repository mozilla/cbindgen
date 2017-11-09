#include <cstdint>
#include <cstdlib>

extern "C" {

enum class Status : uint32_t {
  Ok = 0,
  Err = 1,
};

struct Dep {
  int32_t a;
  float b;
};

struct Foo_i32 {
  int32_t a;
  int32_t b;
  Dep c;
};

typedef Foo_i32 IntFoo;

struct Foo_f64 {
  double a;
  double b;
  Dep c;
};

typedef Foo_f64 DoubleFoo;

typedef int32_t Unit;

typedef Status SpecialStatus;

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

} // extern "C"

template<typename X>
struct Foo;

template<>
struct Foo<double> : public Foo_f64 {

};

template<>
struct Foo<int32_t> : public Foo_i32 {

};

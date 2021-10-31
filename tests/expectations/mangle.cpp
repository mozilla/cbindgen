#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class Bar {
  BarSome,
  BarThing,
};

template<typename T>
struct Foo {
  T a;
};

using Boo = Foo<uint8_t>;

template<typename T>
struct Dog {
  enum class Tag {
    DogWoof,
  };

  struct DogWoof_Body {
    T _0;
  };

  Tag tag;
  union {
    DogWoof_Body woof;
  };
};

extern "C" {

void root(Boo x, Bar y, Dog<Foo<uint8_t>> z);

} // extern "C"

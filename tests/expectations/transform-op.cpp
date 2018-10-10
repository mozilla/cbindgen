#include <cstdint>
#include <cstdlib>

template<typename T>
struct StylePoint {
  T x;
  T y;
};

template<typename T>
union StyleFoo {
  enum class Tag : uint8_t {
    Foo,
    Bar,
    Baz,
    Bazz,
  };

  struct Foo_Body {
    Tag tag;
    int32_t x;
    StylePoint<T> y;
    StylePoint<float> z;
  };

  struct Bar_Body {
    Tag tag;
    T _0;
  };

  struct Baz_Body {
    Tag tag;
    StylePoint<T> _0;
  };

  struct {
    Tag tag;
  };
  Foo_Body foo;
  Bar_Body bar;
  Baz_Body baz;
};

extern "C" {

void foo(const StyleFoo<int32_t> *foo);

} // extern "C"

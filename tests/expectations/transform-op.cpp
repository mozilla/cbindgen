#include <cstdint>
#include <cstdlib>

template<typename T>
struct StylePoint {
  StyleT x;
  StyleT y;
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
    StylePoint<StyleT> y;
    StylePoint<float> z;
  };

  struct Bar_Body {
    Tag tag;
    StyleT _0;
  };

  struct Baz_Body {
    Tag tag;
    StylePoint<StyleT> _0;
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

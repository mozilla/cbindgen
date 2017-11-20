#include <cstdint>
#include <cstdlib>

enum class A : uint32_t {
  a1 = 0,
  a2 = 2,
  a3 = 3,
  a4 = 5,
};

enum class B : uint16_t {
  b1 = 0,
  b2 = 2,
  b3 = 3,
  b4 = 5,
};

enum class C : uint8_t {
  c1 = 0,
  c2 = 2,
  c3 = 3,
  c4 = 5,
};

enum class D : uintptr_t {
  d1 = 0,
  d2 = 2,
  d3 = 3,
  d4 = 5,
};

enum class E : intptr_t {
  e1 = 0,
  e2 = 2,
  e3 = 3,
  e4 = 5,
};

union F {
  enum class Tag : uint8_t {
    Foo = 0,
    Bar = 1,
    Baz = 2,
  };

  struct Foo_Body {
    Tag tag;
    int16_t _0;
  };

  struct Bar_Body {
    Tag tag;
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  Foo_Body foo;
  Bar_Body bar;
};

struct G {
  enum class Tag {
    G_Foo = 0,
    G_Bar = 1,
    G_Baz = 2,
  };

  struct G_Foo_Body {
    int16_t _0;
  };

  struct G_Bar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    G_Foo_Body foo;
    G_Bar_Body bar;
  };
};

struct Opaque;

extern "C" {

void root(Opaque *o, A a, B b, C c, D d, E e, F f, G g);

} // extern "C"

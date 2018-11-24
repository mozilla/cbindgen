#include <cstdint>
#include <cstdlib>

enum class A : uint32_t {
  a1 = 0,
  a2 = 2,
  a3,
  a4 = 5,
};

enum class B : uint16_t {
  b1 = 0,
  b2 = 2,
  b3,
  b4 = 5,
};

enum class C : uint8_t {
  c1 = 0,
  c2 = 2,
  c3,
  c4 = 5,
};

enum class D : uintptr_t {
  d1 = 0,
  d2 = 2,
  d3,
  d4 = 5,
};

enum class E : intptr_t {
  e1 = 0,
  e2 = 2,
  e3,
  e4 = 5,
};

enum class K {
  k1,
  k2,
  k3,
  k4,
};

enum class L : int8_t {
  l1 = -1,
  l2 = 0,
  l3 = 1,
};

struct I;

struct J;

struct Opaque;

union F {
  enum class Tag : uint8_t {
    Foo,
    Bar,
    Baz,
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

  struct {
    Tag tag;
  };
  Foo_Body foo;
  Bar_Body bar;
};

struct G {
  enum class Tag {
    G_Foo,
    G_Bar,
    G_Baz,
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

struct H {
  enum class Tag : uint8_t {
    H_Foo,
    H_Bar,
    H_Baz,
  };

  struct H_Foo_Body {
    int16_t _0;
  };

  struct H_Bar_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    H_Foo_Body foo;
    H_Bar_Body bar;
  };
};

extern "C" {

void root(Opaque *o, A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l);

} // extern "C"

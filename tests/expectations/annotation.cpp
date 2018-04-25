#include <cstdint>
#include <cstdlib>

enum class C : uint32_t {
  X = 2,
  Y,
};

struct A {
  int32_t m0;

  A(int32_t const& aM0)
    : m0(aM0)
  {}

  bool operator<(const A& other) const {
    return m0 < other.m0;
  }
  bool operator<=(const A& other) const {
    return m0 <= other.m0;
  }
};

struct B {
  int32_t x;
  float y;
};

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

  static F Foo(int16_t const& a0) {
    F result;
    result.foo._0 = a0;
    result.tag = Tag::Foo;
    return result;
  }

  static F Bar(uint8_t const& aX,
               int16_t const& aY) {
    F result;
    result.bar.x = aX;
    result.bar.y = aY;
    result.tag = Tag::Bar;
    return result;
  }

  static F Baz() {
    F result;
    result.tag = Tag::Baz;
    return result;
  }

  bool IsFoo() const {
    return tag == Tag::Foo;
  }

  bool IsBar() const {
    return tag == Tag::Bar;
  }

  bool IsBaz() const {
    return tag == Tag::Baz;
  }
};

struct H {
  enum class Tag : uint8_t {
    Hello,
    There,
    Everyone,
  };

  struct Hello_Body {
    int16_t _0;
  };

  struct There_Body {
    uint8_t x;
    int16_t y;
  };

  Tag tag;
  union {
    Hello_Body hello;
    There_Body there;
  };

  static H Hello(int16_t const& a0) {
    H result;
    result.hello._0 = a0;
    result.tag = Tag::Hello;
    return result;
  }

  static H There(uint8_t const& aX,
                 int16_t const& aY) {
    H result;
    result.there.x = aX;
    result.there.y = aY;
    result.tag = Tag::There;
    return result;
  }

  static H Everyone() {
    H result;
    result.tag = Tag::Everyone;
    return result;
  }

  bool IsHello() const {
    return tag == Tag::Hello;
  }

  bool IsThere() const {
    return tag == Tag::There;
  }

  bool IsEveryone() const {
    return tag == Tag::Everyone;
  }
};

extern "C" {

void root(A x, B y, C z, F f, H h);

} // extern "C"

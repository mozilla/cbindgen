#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

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

  static F Foo(const int16_t &a0) {
    F result;
    ::new (&result.foo._0) (int16_t)(a0);
    result.tag = Tag::Foo;
    return result;
  }

  bool IsFoo() const {
    return tag == Tag::Foo;
  }

  static F Bar(const uint8_t &aX,
               const int16_t &aY) {
    F result;
    ::new (&result.bar.x) (uint8_t)(aX);
    ::new (&result.bar.y) (int16_t)(aY);
    result.tag = Tag::Bar;
    return result;
  }

  bool IsBar() const {
    return tag == Tag::Bar;
  }

  static F Baz() {
    F result;
    result.tag = Tag::Baz;
    return result;
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

  static H Hello(const int16_t &a0) {
    H result;
    ::new (&result.hello._0) (int16_t)(a0);
    result.tag = Tag::Hello;
    return result;
  }

  bool IsHello() const {
    return tag == Tag::Hello;
  }

  static H There(const uint8_t &aX,
                 const int16_t &aY) {
    H result;
    ::new (&result.there.x) (uint8_t)(aX);
    ::new (&result.there.y) (int16_t)(aY);
    result.tag = Tag::There;
    return result;
  }

  bool IsThere() const {
    return tag == Tag::There;
  }

  static H Everyone() {
    H result;
    result.tag = Tag::Everyone;
    return result;
  }

  bool IsEveryone() const {
    return tag == Tag::Everyone;
  }
};

extern "C" {

void root(A x, B y, C z, F f, H h);

} // extern "C"

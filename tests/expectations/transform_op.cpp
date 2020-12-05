#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include <cassert>

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

  struct {
    Tag tag;
  };
  Foo_Body foo;
  struct {
    Tag bar_tag;
    T bar;
  };
  struct {
    Tag baz_tag;
    StylePoint<T> baz;
  };

  static StyleFoo Foo(const int32_t &x,
                      const StylePoint<T> &y,
                      const StylePoint<float> &z) {
    StyleFoo result;
    ::new (&result.foo.x) (int32_t)(x);
    ::new (&result.foo.y) (StylePoint<T>)(y);
    ::new (&result.foo.z) (StylePoint<float>)(z);
    result.tag = Tag::Foo;
    return result;
  }

  bool IsFoo() const {
    return tag == Tag::Foo;
  }

  const Foo_Body& AsFoo() const {
    assert(IsFoo());
    return foo;
  }

  Foo_Body& AsFoo() {
    assert(IsFoo());
    return foo;
  }

  static StyleFoo Bar(const T &bar) {
    StyleFoo result;
    ::new (&result.bar) (T)(bar);
    result.tag = Tag::Bar;
    return result;
  }

  bool IsBar() const {
    return tag == Tag::Bar;
  }

  const T& AsBar() const {
    assert(IsBar());
    return bar;
  }

  T& AsBar() {
    assert(IsBar());
    return bar;
  }

  static StyleFoo Baz(const StylePoint<T> &baz) {
    StyleFoo result;
    ::new (&result.baz) (StylePoint<T>)(baz);
    result.tag = Tag::Baz;
    return result;
  }

  bool IsBaz() const {
    return tag == Tag::Baz;
  }

  const StylePoint<T>& AsBaz() const {
    assert(IsBaz());
    return baz;
  }

  StylePoint<T>& AsBaz() {
    assert(IsBaz());
    return baz;
  }

  static StyleFoo Bazz() {
    StyleFoo result;
    result.tag = Tag::Bazz;
    return result;
  }

  bool IsBazz() const {
    return tag == Tag::Bazz;
  }
};

template<typename T>
struct StyleBar {
  enum class Tag {
    Bar1,
    Bar2,
    Bar3,
    Bar4,
  };

  struct StyleBar1_Body {
    int32_t x;
    StylePoint<T> y;
    StylePoint<float> z;
    int32_t (*u)(int32_t);
  };

  Tag tag;
  union {
    StyleBar1_Body bar1;
    struct {
      T bar2;
    };
    struct {
      StylePoint<T> bar3;
    };
  };

  static StyleBar Bar1(const int32_t &x,
                       const StylePoint<T> &y,
                       const StylePoint<float> &z,
                       int32_t (*&u)(int32_t)) {
    StyleBar result;
    ::new (&result.bar1.x) (int32_t)(x);
    ::new (&result.bar1.y) (StylePoint<T>)(y);
    ::new (&result.bar1.z) (StylePoint<float>)(z);
    ::new (&result.bar1.u) (int32_t(*)(int32_t))(u);
    result.tag = Tag::Bar1;
    return result;
  }

  bool IsBar1() const {
    return tag == Tag::Bar1;
  }

  const StyleBar1_Body& AsBar1() const {
    assert(IsBar1());
    return bar1;
  }

  StyleBar1_Body& AsBar1() {
    assert(IsBar1());
    return bar1;
  }

  static StyleBar Bar2(const T &bar2) {
    StyleBar result;
    ::new (&result.bar2) (T)(bar2);
    result.tag = Tag::Bar2;
    return result;
  }

  bool IsBar2() const {
    return tag == Tag::Bar2;
  }

  const T& AsBar2() const {
    assert(IsBar2());
    return bar2;
  }

  T& AsBar2() {
    assert(IsBar2());
    return bar2;
  }

  static StyleBar Bar3(const StylePoint<T> &bar3) {
    StyleBar result;
    ::new (&result.bar3) (StylePoint<T>)(bar3);
    result.tag = Tag::Bar3;
    return result;
  }

  bool IsBar3() const {
    return tag == Tag::Bar3;
  }

  const StylePoint<T>& AsBar3() const {
    assert(IsBar3());
    return bar3;
  }

  StylePoint<T>& AsBar3() {
    assert(IsBar3());
    return bar3;
  }

  static StyleBar Bar4() {
    StyleBar result;
    result.tag = Tag::Bar4;
    return result;
  }

  bool IsBar4() const {
    return tag == Tag::Bar4;
  }
};

union StyleBaz {
  enum class Tag : uint8_t {
    Baz1,
    Baz2,
    Baz3,
  };

  struct {
    Tag tag;
  };
  struct {
    Tag baz1_tag;
    StyleBar<uint32_t> baz1;
  };
  struct {
    Tag baz2_tag;
    StylePoint<int32_t> baz2;
  };

  static StyleBaz Baz1(const StyleBar<uint32_t> &baz1) {
    StyleBaz result;
    ::new (&result.baz1) (StyleBar<uint32_t>)(baz1);
    result.tag = Tag::Baz1;
    return result;
  }

  bool IsBaz1() const {
    return tag == Tag::Baz1;
  }

  const StyleBar<uint32_t>& AsBaz1() const {
    assert(IsBaz1());
    return baz1;
  }

  StyleBar<uint32_t>& AsBaz1() {
    assert(IsBaz1());
    return baz1;
  }

  static StyleBaz Baz2(const StylePoint<int32_t> &baz2) {
    StyleBaz result;
    ::new (&result.baz2) (StylePoint<int32_t>)(baz2);
    result.tag = Tag::Baz2;
    return result;
  }

  bool IsBaz2() const {
    return tag == Tag::Baz2;
  }

  const StylePoint<int32_t>& AsBaz2() const {
    assert(IsBaz2());
    return baz2;
  }

  StylePoint<int32_t>& AsBaz2() {
    assert(IsBaz2());
    return baz2;
  }

  static StyleBaz Baz3() {
    StyleBaz result;
    result.tag = Tag::Baz3;
    return result;
  }

  bool IsBaz3() const {
    return tag == Tag::Baz3;
  }
};

struct StyleTaz {
  enum class Tag : uint8_t {
    Taz1,
    Taz2,
    Taz3,
  };

  Tag tag;
  union {
    struct {
      StyleBar<uint32_t> taz1;
    };
    struct {
      StyleBaz taz2;
    };
  };

  static StyleTaz Taz1(const StyleBar<uint32_t> &taz1) {
    StyleTaz result;
    ::new (&result.taz1) (StyleBar<uint32_t>)(taz1);
    result.tag = Tag::Taz1;
    return result;
  }

  bool IsTaz1() const {
    return tag == Tag::Taz1;
  }

  const StyleBar<uint32_t>& AsTaz1() const {
    assert(IsTaz1());
    return taz1;
  }

  StyleBar<uint32_t>& AsTaz1() {
    assert(IsTaz1());
    return taz1;
  }

  static StyleTaz Taz2(const StyleBaz &taz2) {
    StyleTaz result;
    ::new (&result.taz2) (StyleBaz)(taz2);
    result.tag = Tag::Taz2;
    return result;
  }

  bool IsTaz2() const {
    return tag == Tag::Taz2;
  }

  const StyleBaz& AsTaz2() const {
    assert(IsTaz2());
    return taz2;
  }

  StyleBaz& AsTaz2() {
    assert(IsTaz2());
    return taz2;
  }

  static StyleTaz Taz3() {
    StyleTaz result;
    result.tag = Tag::Taz3;
    return result;
  }

  bool IsTaz3() const {
    return tag == Tag::Taz3;
  }
};

extern "C" {

void foo(const StyleFoo<int32_t> *foo,
         const StyleBar<int32_t> *bar,
         const StyleBaz *baz,
         const StyleTaz *taz);

} // extern "C"

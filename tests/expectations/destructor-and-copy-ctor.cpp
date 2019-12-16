#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

enum class FillRule : uint8_t {
  A,
  B,
};

/// This will have a destructor manually implemented via variant_body, and
/// similarly a Drop impl in Rust.
template<typename T>
struct OwnedSlice {
  uintptr_t len;
  T *ptr;
  ~OwnedSlice() {}
};

template<typename LengthPercentage>
struct Polygon {
  FillRule fill;
  OwnedSlice<LengthPercentage> coordinates;
};

template<typename T>
struct Foo {
  enum class Tag : uint8_t {
    Bar,
    Polygon1,
    Slice1,
    Slice2,
    Slice3,
    Slice4,
  };

  struct Polygon1_Body {
    Polygon<T> _0;
  };

  struct Slice1_Body {
    OwnedSlice<T> _0;
  };

  struct Slice2_Body {
    OwnedSlice<int32_t> _0;
  };

  struct Slice3_Body {
    FillRule fill;
    OwnedSlice<T> coords;
  };

  struct Slice4_Body {
    FillRule fill;
    OwnedSlice<int32_t> coords;
  };

  Tag tag;
  union {
    Polygon1_Body polygon1;
    Slice1_Body slice1;
    Slice2_Body slice2;
    Slice3_Body slice3;
    Slice4_Body slice4;
  };

  static Foo Bar() {
    Foo result;
    result.tag = Tag::Bar;
    return result;
  }

  static Foo Polygon1(const Polygon<T> &a0) {
    Foo result;
    ::new (&result.polygon1._0) (Polygon<T>)(a0);
    result.tag = Tag::Polygon1;
    return result;
  }

  static Foo Slice1(const OwnedSlice<T> &a0) {
    Foo result;
    ::new (&result.slice1._0) (OwnedSlice<T>)(a0);
    result.tag = Tag::Slice1;
    return result;
  }

  static Foo Slice2(const OwnedSlice<int32_t> &a0) {
    Foo result;
    ::new (&result.slice2._0) (OwnedSlice<int32_t>)(a0);
    result.tag = Tag::Slice2;
    return result;
  }

  static Foo Slice3(const FillRule &aFill,
                    const OwnedSlice<T> &aCoords) {
    Foo result;
    ::new (&result.slice3.fill) (FillRule)(aFill);
    ::new (&result.slice3.coords) (OwnedSlice<T>)(aCoords);
    result.tag = Tag::Slice3;
    return result;
  }

  static Foo Slice4(const FillRule &aFill,
                    const OwnedSlice<int32_t> &aCoords) {
    Foo result;
    ::new (&result.slice4.fill) (FillRule)(aFill);
    ::new (&result.slice4.coords) (OwnedSlice<int32_t>)(aCoords);
    result.tag = Tag::Slice4;
    return result;
  }

  bool IsBar() const {
    return tag == Tag::Bar;
  }

  bool IsPolygon1() const {
    return tag == Tag::Polygon1;
  }

  bool IsSlice1() const {
    return tag == Tag::Slice1;
  }

  bool IsSlice2() const {
    return tag == Tag::Slice2;
  }

  bool IsSlice3() const {
    return tag == Tag::Slice3;
  }

  bool IsSlice4() const {
    return tag == Tag::Slice4;
  }

  private:
  Foo() {

  }
  public:


  ~Foo() {
    switch (tag) {
      case Tag::Polygon1: polygon1.~Polygon1_Body(); break;
      case Tag::Slice1: slice1.~Slice1_Body(); break;
      case Tag::Slice2: slice2.~Slice2_Body(); break;
      case Tag::Slice3: slice3.~Slice3_Body(); break;
      case Tag::Slice4: slice4.~Slice4_Body(); break;
      default: break;
    }
  }

  Foo(const Foo& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Polygon1: ::new (&polygon1) (Polygon1_Body)(other.polygon1); break;
      case Tag::Slice1: ::new (&slice1) (Slice1_Body)(other.slice1); break;
      case Tag::Slice2: ::new (&slice2) (Slice2_Body)(other.slice2); break;
      case Tag::Slice3: ::new (&slice3) (Slice3_Body)(other.slice3); break;
      case Tag::Slice4: ::new (&slice4) (Slice4_Body)(other.slice4); break;
      default: break;
    }
  }
  Foo& operator=(const Foo& other) {
    if (this != &other) {
      this->~Foo();
      new (this) Foo(other);
    }
    return *this;
  }
};

template<typename T>
union Baz {
  enum class Tag : uint8_t {
    Bar2,
    Polygon21,
    Slice21,
    Slice22,
    Slice23,
    Slice24,
  };

  struct Polygon21_Body {
    Tag tag;
    Polygon<T> _0;
  };

  struct Slice21_Body {
    Tag tag;
    OwnedSlice<T> _0;
  };

  struct Slice22_Body {
    Tag tag;
    OwnedSlice<int32_t> _0;
  };

  struct Slice23_Body {
    Tag tag;
    FillRule fill;
    OwnedSlice<T> coords;
  };

  struct Slice24_Body {
    Tag tag;
    FillRule fill;
    OwnedSlice<int32_t> coords;
  };

  struct {
    Tag tag;
  };
  Polygon21_Body polygon21;
  Slice21_Body slice21;
  Slice22_Body slice22;
  Slice23_Body slice23;
  Slice24_Body slice24;

  static Baz Bar2() {
    Baz result;
    result.tag = Tag::Bar2;
    return result;
  }

  static Baz Polygon21(const Polygon<T> &a0) {
    Baz result;
    ::new (&result.polygon21._0) (Polygon<T>)(a0);
    result.tag = Tag::Polygon21;
    return result;
  }

  static Baz Slice21(const OwnedSlice<T> &a0) {
    Baz result;
    ::new (&result.slice21._0) (OwnedSlice<T>)(a0);
    result.tag = Tag::Slice21;
    return result;
  }

  static Baz Slice22(const OwnedSlice<int32_t> &a0) {
    Baz result;
    ::new (&result.slice22._0) (OwnedSlice<int32_t>)(a0);
    result.tag = Tag::Slice22;
    return result;
  }

  static Baz Slice23(const FillRule &aFill,
                     const OwnedSlice<T> &aCoords) {
    Baz result;
    ::new (&result.slice23.fill) (FillRule)(aFill);
    ::new (&result.slice23.coords) (OwnedSlice<T>)(aCoords);
    result.tag = Tag::Slice23;
    return result;
  }

  static Baz Slice24(const FillRule &aFill,
                     const OwnedSlice<int32_t> &aCoords) {
    Baz result;
    ::new (&result.slice24.fill) (FillRule)(aFill);
    ::new (&result.slice24.coords) (OwnedSlice<int32_t>)(aCoords);
    result.tag = Tag::Slice24;
    return result;
  }

  bool IsBar2() const {
    return tag == Tag::Bar2;
  }

  bool IsPolygon21() const {
    return tag == Tag::Polygon21;
  }

  bool IsSlice21() const {
    return tag == Tag::Slice21;
  }

  bool IsSlice22() const {
    return tag == Tag::Slice22;
  }

  bool IsSlice23() const {
    return tag == Tag::Slice23;
  }

  bool IsSlice24() const {
    return tag == Tag::Slice24;
  }

  private:
  Baz() {

  }
  public:


  ~Baz() {
    switch (tag) {
      case Tag::Polygon21: polygon21.~Polygon21_Body(); break;
      case Tag::Slice21: slice21.~Slice21_Body(); break;
      case Tag::Slice22: slice22.~Slice22_Body(); break;
      case Tag::Slice23: slice23.~Slice23_Body(); break;
      case Tag::Slice24: slice24.~Slice24_Body(); break;
      default: break;
    }
  }

  Baz(const Baz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Polygon21: ::new (&polygon21) (Polygon21_Body)(other.polygon21); break;
      case Tag::Slice21: ::new (&slice21) (Slice21_Body)(other.slice21); break;
      case Tag::Slice22: ::new (&slice22) (Slice22_Body)(other.slice22); break;
      case Tag::Slice23: ::new (&slice23) (Slice23_Body)(other.slice23); break;
      case Tag::Slice24: ::new (&slice24) (Slice24_Body)(other.slice24); break;
      default: break;
    }
  }
  Baz& operator=(const Baz& other) {
    if (this != &other) {
      this->~Baz();
      new (this) Baz(other);
    }
    return *this;
  }
};

union Taz {
  enum class Tag : uint8_t {
    Bar3,
    Taz1,
    Taz3,
  };

  struct Taz1_Body {
    Tag tag;
    int32_t _0;
  };

  struct Taz3_Body {
    Tag tag;
    OwnedSlice<int32_t> _0;
  };

  struct {
    Tag tag;
  };
  Taz1_Body taz1;
  Taz3_Body taz3;

  static Taz Bar3() {
    Taz result;
    result.tag = Tag::Bar3;
    return result;
  }

  static Taz Taz1(const int32_t &a0) {
    Taz result;
    ::new (&result.taz1._0) (int32_t)(a0);
    result.tag = Tag::Taz1;
    return result;
  }

  static Taz Taz3(const OwnedSlice<int32_t> &a0) {
    Taz result;
    ::new (&result.taz3._0) (OwnedSlice<int32_t>)(a0);
    result.tag = Tag::Taz3;
    return result;
  }

  bool IsBar3() const {
    return tag == Tag::Bar3;
  }

  bool IsTaz1() const {
    return tag == Tag::Taz1;
  }

  bool IsTaz3() const {
    return tag == Tag::Taz3;
  }

  private:
  Taz() {

  }
  public:


  ~Taz() {
    switch (tag) {
      case Tag::Taz1: taz1.~Taz1_Body(); break;
      case Tag::Taz3: taz3.~Taz3_Body(); break;
      default: break;
    }
  }

  Taz(const Taz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Taz1: ::new (&taz1) (Taz1_Body)(other.taz1); break;
      case Tag::Taz3: ::new (&taz3) (Taz3_Body)(other.taz3); break;
      default: break;
    }
  }
  Taz& operator=(const Taz& other) {
    if (this != &other) {
      this->~Taz();
      new (this) Taz(other);
    }
    return *this;
  }
};

union Tazz {
  enum class Tag : uint8_t {
    Bar4,
    Taz2,
  };

  struct Taz2_Body {
    Tag tag;
    int32_t _0;
  };

  struct {
    Tag tag;
  };
  Taz2_Body taz2;

  static Tazz Bar4() {
    Tazz result;
    result.tag = Tag::Bar4;
    return result;
  }

  static Tazz Taz2(const int32_t &a0) {
    Tazz result;
    ::new (&result.taz2._0) (int32_t)(a0);
    result.tag = Tag::Taz2;
    return result;
  }

  bool IsBar4() const {
    return tag == Tag::Bar4;
  }

  bool IsTaz2() const {
    return tag == Tag::Taz2;
  }

  private:
  Tazz() {

  }
  public:

};

extern "C" {

void root(const Foo<uint32_t> *a, const Baz<int32_t> *b, const Taz *c, Tazz d);

} // extern "C"

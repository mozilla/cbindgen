#define NOINLINE __attribute__((noinline))
#define NODISCARD [[nodiscard]]


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
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
    struct {
      Polygon<T> polygon1;
    };
    struct {
      OwnedSlice<T> slice1;
    };
    struct {
      OwnedSlice<int32_t> slice2;
    };
    Slice3_Body slice3;
    Slice4_Body slice4;
  };

  static Foo Bar() {
    Foo result;
    result.tag = Tag::Bar;
    return result;
  }

  bool IsBar() const {
    return tag == Tag::Bar;
  }

  static Foo Polygon1(const Polygon<T> &polygon1) {
    Foo result;
    ::new (&result.polygon1) (Polygon<T>)(polygon1);
    result.tag = Tag::Polygon1;
    return result;
  }

  bool IsPolygon1() const {
    return tag == Tag::Polygon1;
  }

  static Foo Slice1(const OwnedSlice<T> &slice1) {
    Foo result;
    ::new (&result.slice1) (OwnedSlice<T>)(slice1);
    result.tag = Tag::Slice1;
    return result;
  }

  bool IsSlice1() const {
    return tag == Tag::Slice1;
  }

  static Foo Slice2(const OwnedSlice<int32_t> &slice2) {
    Foo result;
    ::new (&result.slice2) (OwnedSlice<int32_t>)(slice2);
    result.tag = Tag::Slice2;
    return result;
  }

  bool IsSlice2() const {
    return tag == Tag::Slice2;
  }

  static Foo Slice3(const FillRule &fill,
                    const OwnedSlice<T> &coords) {
    Foo result;
    ::new (&result.slice3.fill) (FillRule)(fill);
    ::new (&result.slice3.coords) (OwnedSlice<T>)(coords);
    result.tag = Tag::Slice3;
    return result;
  }

  bool IsSlice3() const {
    return tag == Tag::Slice3;
  }

  static Foo Slice4(const FillRule &fill,
                    const OwnedSlice<int32_t> &coords) {
    Foo result;
    ::new (&result.slice4.fill) (FillRule)(fill);
    ::new (&result.slice4.coords) (OwnedSlice<int32_t>)(coords);
    result.tag = Tag::Slice4;
    return result;
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
      case Tag::Polygon1: polygon1.~Polygon<T>();break;
      case Tag::Slice1: slice1.~OwnedSlice<T>();break;
      case Tag::Slice2: slice2.~OwnedSlice<int32_t>();break;
      case Tag::Slice3: slice3.~Slice3_Body();break;
      case Tag::Slice4: slice4.~Slice4_Body();break;
      default: break;
    }
  }

  Foo(const Foo& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Polygon1: new (&polygon1) (decltype(polygon1))(other.polygon1); break;
      case Tag::Slice1: new (&slice1) (decltype(slice1))(other.slice1); break;
      case Tag::Slice2: new (&slice2) (decltype(slice2))(other.slice2); break;
      case Tag::Slice3: new (&slice3) (Slice3_Body)(other.slice3); break;
      case Tag::Slice4: new (&slice4) (Slice4_Body)(other.slice4); break;
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
  struct {
    Tag polygon21_tag;
    Polygon<T> polygon21;
  };
  struct {
    Tag slice21_tag;
    OwnedSlice<T> slice21;
  };
  struct {
    Tag slice22_tag;
    OwnedSlice<int32_t> slice22;
  };
  Slice23_Body slice23;
  Slice24_Body slice24;

  static Baz Bar2() {
    Baz result;
    result.tag = Tag::Bar2;
    return result;
  }

  bool IsBar2() const {
    return tag == Tag::Bar2;
  }

  static Baz Polygon21(const Polygon<T> &polygon21) {
    Baz result;
    ::new (&result.polygon21) (Polygon<T>)(polygon21);
    result.tag = Tag::Polygon21;
    return result;
  }

  bool IsPolygon21() const {
    return tag == Tag::Polygon21;
  }

  static Baz Slice21(const OwnedSlice<T> &slice21) {
    Baz result;
    ::new (&result.slice21) (OwnedSlice<T>)(slice21);
    result.tag = Tag::Slice21;
    return result;
  }

  bool IsSlice21() const {
    return tag == Tag::Slice21;
  }

  static Baz Slice22(const OwnedSlice<int32_t> &slice22) {
    Baz result;
    ::new (&result.slice22) (OwnedSlice<int32_t>)(slice22);
    result.tag = Tag::Slice22;
    return result;
  }

  bool IsSlice22() const {
    return tag == Tag::Slice22;
  }

  static Baz Slice23(const FillRule &fill,
                     const OwnedSlice<T> &coords) {
    Baz result;
    ::new (&result.slice23.fill) (FillRule)(fill);
    ::new (&result.slice23.coords) (OwnedSlice<T>)(coords);
    result.tag = Tag::Slice23;
    return result;
  }

  bool IsSlice23() const {
    return tag == Tag::Slice23;
  }

  static Baz Slice24(const FillRule &fill,
                     const OwnedSlice<int32_t> &coords) {
    Baz result;
    ::new (&result.slice24.fill) (FillRule)(fill);
    ::new (&result.slice24.coords) (OwnedSlice<int32_t>)(coords);
    result.tag = Tag::Slice24;
    return result;
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
      case Tag::Polygon21: polygon21.~Polygon<T>();break;
      case Tag::Slice21: slice21.~OwnedSlice<T>();break;
      case Tag::Slice22: slice22.~OwnedSlice<int32_t>();break;
      case Tag::Slice23: slice23.~Slice23_Body();break;
      case Tag::Slice24: slice24.~Slice24_Body();break;
      default: break;
    }
  }

  Baz(const Baz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Polygon21:
        new (&polygon21_tag) (decltype(polygon21_tag))(other.polygon21_tag);
        new (&polygon21) (decltype(polygon21))(other.polygon21);
      break;
      case Tag::Slice21:
        new (&slice21_tag) (decltype(slice21_tag))(other.slice21_tag);
        new (&slice21) (decltype(slice21))(other.slice21);
      break;
      case Tag::Slice22:
        new (&slice22_tag) (decltype(slice22_tag))(other.slice22_tag);
        new (&slice22) (decltype(slice22))(other.slice22);
      break;
      case Tag::Slice23: new (&slice23) (Slice23_Body)(other.slice23); break;
      case Tag::Slice24: new (&slice24) (Slice24_Body)(other.slice24); break;
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

  struct {
    Tag tag;
  };
  struct {
    Tag taz1_tag;
    int32_t taz1;
  };
  struct {
    Tag taz3_tag;
    OwnedSlice<int32_t> taz3;
  };

  static Taz Bar3() {
    Taz result;
    result.tag = Tag::Bar3;
    return result;
  }

  bool IsBar3() const {
    return tag == Tag::Bar3;
  }

  static Taz Taz1(const int32_t &taz1) {
    Taz result;
    ::new (&result.taz1) (int32_t)(taz1);
    result.tag = Tag::Taz1;
    return result;
  }

  bool IsTaz1() const {
    return tag == Tag::Taz1;
  }

  static Taz Taz3(const OwnedSlice<int32_t> &taz3) {
    Taz result;
    ::new (&result.taz3) (OwnedSlice<int32_t>)(taz3);
    result.tag = Tag::Taz3;
    return result;
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
      case Tag::Taz1: taz1.~int32_t();break;
      case Tag::Taz3: taz3.~OwnedSlice<int32_t>();break;
      default: break;
    }
  }

  Taz(const Taz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Taz1:
        new (&taz1_tag) (decltype(taz1_tag))(other.taz1_tag);
        new (&taz1) (decltype(taz1))(other.taz1);
      break;
      case Tag::Taz3:
        new (&taz3_tag) (decltype(taz3_tag))(other.taz3_tag);
        new (&taz3) (decltype(taz3))(other.taz3);
      break;
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

  struct {
    Tag tag;
  };
  struct {
    Tag taz2_tag;
    int32_t taz2;
  };

  static Tazz Bar4() {
    Tazz result;
    result.tag = Tag::Bar4;
    return result;
  }

  bool IsBar4() const {
    return tag == Tag::Bar4;
  }

  static Tazz Taz2(const int32_t &taz2) {
    Tazz result;
    ::new (&result.taz2) (int32_t)(taz2);
    result.tag = Tag::Taz2;
    return result;
  }

  bool IsTaz2() const {
    return tag == Tag::Taz2;
  }

  private:
  Tazz() {

  }
  public:

};

union Tazzz {
  enum class Tag : uint8_t {
    Bar5,
    Taz5,
  };

  struct {
    Tag tag;
  };
  struct {
    Tag taz5_tag;
    int32_t taz5;
  };

  static Tazzz Bar5() {
    Tazzz result;
    result.tag = Tag::Bar5;
    return result;
  }

  bool IsBar5() const {
    return tag == Tag::Bar5;
  }

  static Tazzz Taz5(const int32_t &taz5) {
    Tazzz result;
    ::new (&result.taz5) (int32_t)(taz5);
    result.tag = Tag::Taz5;
    return result;
  }

  bool IsTaz5() const {
    return tag == Tag::Taz5;
  }

  private:
  Tazzz() {

  }
  public:


  ~Tazzz() {
    switch (tag) {
      case Tag::Taz5: taz5.~int32_t();break;
      default: break;
    }
  }

  Tazzz(const Tazzz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Taz5:
        new (&taz5_tag) (decltype(taz5_tag))(other.taz5_tag);
        new (&taz5) (decltype(taz5))(other.taz5);
      break;
      default: break;
    }
  }
};

union Tazzzz {
  enum class Tag : uint8_t {
    Taz6,
    Taz7,
  };

  struct {
    Tag tag;
  };
  struct {
    Tag taz6_tag;
    int32_t taz6;
  };
  struct {
    Tag taz7_tag;
    uint32_t taz7;
  };

  static Tazzzz Taz6(const int32_t &taz6) {
    Tazzzz result;
    ::new (&result.taz6) (int32_t)(taz6);
    result.tag = Tag::Taz6;
    return result;
  }

  bool IsTaz6() const {
    return tag == Tag::Taz6;
  }

  static Tazzzz Taz7(const uint32_t &taz7) {
    Tazzzz result;
    ::new (&result.taz7) (uint32_t)(taz7);
    result.tag = Tag::Taz7;
    return result;
  }

  bool IsTaz7() const {
    return tag == Tag::Taz7;
  }

  private:
  Tazzzz() {

  }
  public:


  ~Tazzzz() {
    switch (tag) {
      case Tag::Taz6: taz6.~int32_t();break;
      case Tag::Taz7: taz7.~uint32_t();break;

    }
  }

  Tazzzz(const Tazzzz& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Taz6:
        new (&taz6_tag) (decltype(taz6_tag))(other.taz6_tag);
        new (&taz6) (decltype(taz6))(other.taz6);
      break;
      case Tag::Taz7:
        new (&taz7_tag) (decltype(taz7_tag))(other.taz7_tag);
        new (&taz7) (decltype(taz7))(other.taz7);
      break;

    }
  }
  Tazzzz& operator=(const Tazzzz& other) {
    if (this != &other) {
      this->~Tazzzz();
      new (this) Tazzzz(other);
    }
    return *this;
  }
};

union Qux {
  enum class Tag : uint8_t {
    Qux1,
    Qux2,
  };

  struct {
    Tag tag;
  };
  struct {
    Tag qux1_tag;
    int32_t qux1;
  };
  struct {
    Tag qux2_tag;
    uint32_t qux2;
  };

  static Qux Qux1(const int32_t &qux1) {
    Qux result;
    ::new (&result.qux1) (int32_t)(qux1);
    result.tag = Tag::Qux1;
    return result;
  }

  bool IsQux1() const {
    return tag == Tag::Qux1;
  }

  static Qux Qux2(const uint32_t &qux2) {
    Qux result;
    ::new (&result.qux2) (uint32_t)(qux2);
    result.tag = Tag::Qux2;
    return result;
  }

  bool IsQux2() const {
    return tag == Tag::Qux2;
  }

  NODISCARD bool operator==(const Qux& other) const {
    if (tag != other.tag) {
      return false;
    }
    switch (tag) {
      case Tag::Qux1: return qux1 == other.qux1;
      case Tag::Qux2: return qux2 == other.qux2;

    }
    return true;
  }

  NODISCARD bool operator!=(const Qux& other) const {
    return !(*this == other);
  }

  private:
  Qux() {

  }
  public:


  NOINLINE ~Qux() {
    switch (tag) {
      case Tag::Qux1: qux1.~int32_t();break;
      case Tag::Qux2: qux2.~uint32_t();break;

    }
  }

  NOINLINE Qux(const Qux& other)
   : tag(other.tag) {
    switch (tag) {
      case Tag::Qux1:
        new (&qux1_tag) (decltype(qux1_tag))(other.qux1_tag);
        new (&qux1) (decltype(qux1))(other.qux1);
      break;
      case Tag::Qux2:
        new (&qux2_tag) (decltype(qux2_tag))(other.qux2_tag);
        new (&qux2) (decltype(qux2))(other.qux2);
      break;

    }
  }
  NOINLINE Qux& operator=(const Qux& other) {
    if (this != &other) {
      this->~Qux();
      new (this) Qux(other);
    }
    return *this;
  }
};

extern "C" {

void root(const Foo<uint32_t> *a,
          const Baz<int32_t> *b,
          const Taz *c,
          Tazz d,
          const Tazzz *e,
          const Tazzzz *f,
          const Qux *g);

} // extern "C"

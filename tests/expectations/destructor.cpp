#include <cstdarg>
#include <cstdint>
#include <cstdlib>

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
};

extern "C" {

void root(const Foo<uint32_t> *a, const Baz<int32_t> *b);

} // extern "C"

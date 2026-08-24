#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Regression test: in C++ output, the generated static variant constructors of a
/// tagged enum must value-initialize their `result` local (`Foo result{};`) so that
/// the payload union is zero-initialized rather than left indeterminate. This avoids
/// false "uninitialized scalar" reports from static analysis for no-payload variants.
///
struct ValueInitEnum {
  enum class Tag : uint8_t {
    /// No payload: `result` is returned with only the tag set, so the union must be
    /// value-initialized.
    Empty,
    /// Payload variant: constructed via placement new over the value-initialized union.
    Number,
    Pair,
  };

  struct Number_Body {
    int32_t _0;
  };

  struct Pair_Body {
    int32_t _0;
    uint64_t _1;
  };

  Tag tag;
  union {
    Number_Body number;
    Pair_Body pair;
  };

  static ValueInitEnum Empty() {
    ValueInitEnum result{};
    result.tag = Tag::Empty;
    return result;
  }

  bool IsEmpty() const {
    return tag == Tag::Empty;
  }

  static ValueInitEnum Number(const int32_t &_0) {
    ValueInitEnum result{};
    ::new (&result.number._0) (int32_t)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  static ValueInitEnum Pair(const int32_t &_0,
                            const uint64_t &_1) {
    ValueInitEnum result{};
    ::new (&result.pair._0) (int32_t)(_0);
    ::new (&result.pair._1) (uint64_t)(_1);
    result.tag = Tag::Pair;
    return result;
  }

  bool IsPair() const {
    return tag == Tag::Pair;
  }
};

extern "C" {

void root(ValueInitEnum e);

}  // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct MyStruct {
  uint32_t int_field;
  T generic_field;
};

template<typename T, typename R>
struct S {
  T x;
  R y;
};

template<typename A, typename B>
struct Either {
  enum class Tag {
    Left,
    Right,
  };

  struct Left_Body {
    A _0;
  };

  struct Right_Body {
    B _0;
  };

  Tag tag;
  union {
    Left_Body left;
    Right_Body right;
  };
};

extern "C" {

MyStruct<void> my_test();

S<uint32_t, void> f();

Either<void, uint8_t> returns_either();

}  // extern "C"

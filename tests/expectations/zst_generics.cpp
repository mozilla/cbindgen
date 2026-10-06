#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
struct StructWithGenArray {
  T gen_array[5];
  uint32_t other_field;
};

template<typename T>
union U {
  T a;
  uint32_t b;
};

template<typename T>
struct E {
  enum class Tag {
    A,
    B,
    C,
  };

  struct A_Body {
    T x;
    uint32_t y;
  };

  struct B_Body {
    T _0;
  };

  Tag tag;
  union {
    A_Body a;
    B_Body b;
  };
};

template<typename T, uintptr_t N>
struct CG {
  T a[N];
  uint32_t b;
};

template<typename T, typename R>
struct Two {
  T t;
  R r;
  uint32_t k;
};

template<typename T>
struct Wrap1 {
  T a;
  uint32_t b;
};

template<typename T>
struct Wrap2 {
  Wrap1<T> w;
  uint16_t c;
};

template<typename T, typename R = void>
struct D2 {
  T t;
  R r;
  uint32_t k;
};

extern "C" {

void use_struct(StructWithGenArray<void> arg);

void use_union(U<void> x);

void use_enum(E<void> x);

void use_cg(CG<void, 3> x);

void use_two(Two<void, void> x, Two<void, uint8_t> y, Two<uint8_t, void> z);

void use_nested(Wrap2<void> x, Wrap1<Wrap1<void>> y);

void use_d2(D2<uint8_t> x, D2<uint8_t, void> y);

}  // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Opaque;

struct Normal {
  int32_t x;
  float y;
};

struct NormalWithZST {
  int32_t x;
  float y;
};

struct TupleRenamed {
  int32_t m0;
  float m1;
};

struct TupleNamed {
  int32_t x;
  float y;
};

struct WithFlexibleArrayMember {
  int32_t x;
  int16_t y[0];
  int8_t z[0];
};

extern "C" {

void root(Opaque *a,
          Normal b,
          NormalWithZST c,
          TupleRenamed d,
          TupleNamed e,
          WithFlexibleArrayMember f);

}  // extern "C"

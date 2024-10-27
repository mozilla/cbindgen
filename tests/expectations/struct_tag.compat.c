#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Opaque *a,
          struct Normal b,
          struct NormalWithZST c,
          struct TupleRenamed d,
          struct TupleNamed e,
          struct WithFlexibleArrayMember f);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

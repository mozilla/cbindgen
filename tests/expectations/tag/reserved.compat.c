#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct A {
  int32_t namespace_;
  float float_;
};

struct B {
  int32_t namespace_;
  float float_;
};

enum C_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  D,
};
#ifndef __cplusplus
typedef uint8_t C_Tag;
#endif // __cplusplus

struct D_Body {
  int32_t namespace_;
  float float_;
};

struct C {
  C_Tag tag;
  union {
    struct D_Body d;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct A a, struct B b, struct C c, int32_t namespace_, float float_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

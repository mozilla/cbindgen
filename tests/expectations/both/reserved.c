#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct A {
  int32_t namespace_;
  float float_;
} A;

typedef struct B {
  int32_t namespace_;
  float float_;
} B;

enum C_Tag {
  D,
};
typedef uint8_t C_Tag;

typedef struct D_Body {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct C {
  C_Tag tag;
  union {
    D_Body d;
  };
} C;

void root(A a, B b, C c, int32_t namespace_, float float_);

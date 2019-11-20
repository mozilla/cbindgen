#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

struct A {
  int32_t namespace_;
  float float_;
};

struct B {
  int32_t namespace_;
  float float_;
};

enum C_Tag {
  D,
};
typedef uint8_t C_Tag;

struct D_Body {
  int32_t namespace_;
  float float_;
};

struct C {
  enum C_Tag tag;
  union {
    struct D_Body d;
  };
};

void root(struct A a, struct B b, struct C c, int32_t namespace_, float float_);

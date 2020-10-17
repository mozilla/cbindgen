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

enum C_Tag {
  D,
};
typedef uint8_t C_Tag;

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

enum E_Tag {
  Double,
  Float,
};
typedef uint8_t E_Tag;

struct Double_Body {
  double _0;
};

struct Float_Body {
  float _0;
};

struct E {
  E_Tag tag;
  union {
    struct Double_Body double_;
    struct Float_Body float_;
  };
};

enum F_Tag {
  double_,
  float_,
};
typedef uint8_t F_Tag;

struct double_Body {
  double _0;
};

struct float_Body {
  float _0;
};

struct F {
  F_Tag tag;
  union {
    struct double_Body double_;
    struct float_Body float_;
  };
};

void root(struct A a,
          struct B b,
          struct C c,
          struct E e,
          struct F f,
          int32_t namespace_,
          float float_);

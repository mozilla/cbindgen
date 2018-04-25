#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

enum C {
  X = 2,
  Y,
};
typedef uint32_t C;

typedef struct {
  int32_t m0;
} A;

typedef struct {
  int32_t x;
  float y;
} B;

enum F_Tag {
  Foo,
  Bar,
  Baz,
};
typedef uint8_t F_Tag;

typedef struct {
  F_Tag tag;
  int16_t _0;
} Foo_Body;

typedef struct {
  F_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef union {
  F_Tag tag;
  Foo_Body foo;
  Bar_Body bar;
} F;

enum H_Tag {
  Hello,
  There,
  Everyone,
};
typedef uint8_t H_Tag;

typedef struct {
  int16_t _0;
} Hello_Body;

typedef struct {
  uint8_t x;
  int16_t y;
} There_Body;

typedef struct {
  H_Tag tag;
  union {
    Hello_Body hello;
    There_Body there;
  };
} H;

void root(A x, B y, C z, F f, H h);

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

struct StylePoint_i32 {
  int32_t x;
  int32_t y;
};

struct StylePoint_f32 {
  float x;
  float y;
};

enum StyleFoo_i32_Tag {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
typedef uint8_t StyleFoo_i32_Tag;

struct StyleFoo_Body_i32 {
  StyleFoo_i32_Tag tag;
  int32_t x;
  struct StylePoint_i32 y;
  struct StylePoint_f32 z;
};

struct StyleBar_Body_i32 {
  StyleFoo_i32_Tag tag;
  int32_t _0;
};

struct StyleBaz_Body_i32 {
  StyleFoo_i32_Tag tag;
  struct StylePoint_i32 _0;
};

union StyleFoo_i32 {
  enum StyleFoo_i32_Tag tag;
  struct StyleFoo_Body_i32 foo;
  struct StyleBar_Body_i32 bar;
  struct StyleBaz_Body_i32 baz;
};

enum StyleBar_i32_Tag {
  Bar1_i32,
  Bar2_i32,
  Bar3_i32,
  Bar4_i32,
};

struct StyleBar1_Body_i32 {
  int32_t x;
  struct StylePoint_i32 y;
  struct StylePoint_f32 z;
};

struct StyleBar2_Body_i32 {
  int32_t _0;
};

struct StyleBar3_Body_i32 {
  struct StylePoint_i32 _0;
};

struct StyleBar_i32 {
  enum StyleBar_i32_Tag tag;
  union {
    struct StyleBar1_Body_i32 bar1;
    struct StyleBar2_Body_i32 bar2;
    struct StyleBar3_Body_i32 bar3;
  };
};

void foo(const union StyleFoo_i32 *foo, const struct StyleBar_i32 *bar);

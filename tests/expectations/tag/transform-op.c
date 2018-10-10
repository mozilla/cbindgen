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

enum StyleFoo_Tag {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
typedef uint8_t StyleFoo_Tag;

struct StyleFoo_Body_i32 {
  StyleFoo_Tag tag;
  int32_t x;
  struct StylePoint_i32 y;
  struct StylePoint_f32 z;
};

struct StyleBar_Body_i32 {
  StyleFoo_Tag tag;
  int32_t _0;
};

struct StyleBaz_Body_i32 {
  StyleFoo_Tag tag;
  struct StylePoint_i32 _0;
};

union StyleFoo_i32 {
  enum StyleFoo_Tag tag;
  struct StyleFoo_Body_i32 foo;
  struct StyleBar_Body_i32 bar;
  struct StyleBaz_Body_i32 baz;
};

void foo(const union StyleFoo_i32 *foo);

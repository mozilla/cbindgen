#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  int32_t x;
  int32_t y;
} StylePoint_i32;

typedef struct {
  float x;
  float y;
} StylePoint_f32;

enum StyleFoo_Tag {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
typedef uint8_t StyleFoo_Tag;

typedef struct {
  StyleFoo_Tag tag;
  int32_t x;
  StylePoint_i32 y;
  StylePoint_f32 z;
} StyleFoo_Body_i32;

typedef struct {
  StyleFoo_Tag tag;
  int32_t _0;
} StyleBar_Body_i32;

typedef struct {
  StyleFoo_Tag tag;
  StylePoint_i32 _0;
} StyleBaz_Body_i32;

typedef union {
  StyleFoo_Tag tag;
  StyleFoo_Body_i32 foo;
  StyleBar_Body_i32 bar;
  StyleBaz_Body_i32 baz;
} StyleFoo_i32;

void foo(const StyleFoo_i32 *foo);

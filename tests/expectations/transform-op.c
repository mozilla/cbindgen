#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t x;
  int32_t y;
} StylePoint_i32;

typedef struct {
  float x;
  float y;
} StylePoint_f32;

enum StyleFoo_i32_Tag {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
typedef uint8_t StyleFoo_i32_Tag;

typedef struct {
  StyleFoo_i32_Tag tag;
  int32_t x;
  StylePoint_i32 y;
  StylePoint_f32 z;
} StyleFoo_Body_i32;

typedef struct {
  StyleFoo_i32_Tag tag;
  int32_t _0;
} StyleBar_Body_i32;

typedef struct {
  StyleFoo_i32_Tag tag;
  StylePoint_i32 _0;
} StyleBaz_Body_i32;

typedef union {
  StyleFoo_i32_Tag tag;
  StyleFoo_Body_i32 foo;
  StyleBar_Body_i32 bar;
  StyleBaz_Body_i32 baz;
} StyleFoo_i32;

typedef enum {
  Bar1_i32,
  Bar2_i32,
  Bar3_i32,
  Bar4_i32,
} StyleBar_i32_Tag;

typedef struct {
  int32_t x;
  StylePoint_i32 y;
  StylePoint_f32 z;
  int32_t (*u)(int32_t);
} StyleBar1_Body_i32;

typedef struct {
  int32_t _0;
} StyleBar2_Body_i32;

typedef struct {
  StylePoint_i32 _0;
} StyleBar3_Body_i32;

typedef struct {
  StyleBar_i32_Tag tag;
  union {
    StyleBar1_Body_i32 bar1;
    StyleBar2_Body_i32 bar2;
    StyleBar3_Body_i32 bar3;
  };
} StyleBar_i32;

typedef struct {
  uint32_t x;
  uint32_t y;
} StylePoint_u32;

typedef enum {
  Bar1_u32,
  Bar2_u32,
  Bar3_u32,
  Bar4_u32,
} StyleBar_u32_Tag;

typedef struct {
  int32_t x;
  StylePoint_u32 y;
  StylePoint_f32 z;
  int32_t (*u)(int32_t);
} StyleBar1_Body_u32;

typedef struct {
  uint32_t _0;
} StyleBar2_Body_u32;

typedef struct {
  StylePoint_u32 _0;
} StyleBar3_Body_u32;

typedef struct {
  StyleBar_u32_Tag tag;
  union {
    StyleBar1_Body_u32 bar1;
    StyleBar2_Body_u32 bar2;
    StyleBar3_Body_u32 bar3;
  };
} StyleBar_u32;

enum StyleBaz_Tag {
  Baz1,
  Baz2,
  Baz3,
};
typedef uint8_t StyleBaz_Tag;

typedef struct {
  StyleBaz_Tag tag;
  StyleBar_u32 _0;
} StyleBaz1_Body;

typedef struct {
  StyleBaz_Tag tag;
  StylePoint_i32 _0;
} StyleBaz2_Body;

typedef union {
  StyleBaz_Tag tag;
  StyleBaz1_Body baz1;
  StyleBaz2_Body baz2;
} StyleBaz;

enum StyleTaz_Tag {
  Taz1,
  Taz2,
  Taz3,
};
typedef uint8_t StyleTaz_Tag;

typedef struct {
  StyleBar_u32 _0;
} StyleTaz1_Body;

typedef struct {
  StyleBaz _0;
} StyleTaz2_Body;

typedef struct {
  StyleTaz_Tag tag;
  union {
    StyleTaz1_Body taz1;
    StyleTaz2_Body taz2;
  };
} StyleTaz;

void foo(const StyleFoo_i32 *foo,
         const StyleBar_i32 *bar,
         const StyleBaz *baz,
         const StyleTaz *taz);

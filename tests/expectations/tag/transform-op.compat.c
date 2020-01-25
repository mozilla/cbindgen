#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct StylePoint_i32 {
  int32_t x;
  int32_t y;
};

struct StylePoint_f32 {
  float x;
  float y;
};

enum StyleFoo_i32_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Foo_i32,
  Bar_i32,
  Baz_i32,
  Bazz_i32,
};
#ifndef __cplusplus
typedef uint8_t StyleFoo_i32_Tag;
#endif // __cplusplus

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
  StyleFoo_i32_Tag tag;
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
  int32_t (*u)(int32_t);
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

struct StylePoint_u32 {
  uint32_t x;
  uint32_t y;
};

enum StyleBar_u32_Tag {
  Bar1_u32,
  Bar2_u32,
  Bar3_u32,
  Bar4_u32,
};

struct StyleBar1_Body_u32 {
  int32_t x;
  struct StylePoint_u32 y;
  struct StylePoint_f32 z;
  int32_t (*u)(int32_t);
};

struct StyleBar2_Body_u32 {
  uint32_t _0;
};

struct StyleBar3_Body_u32 {
  struct StylePoint_u32 _0;
};

struct StyleBar_u32 {
  enum StyleBar_u32_Tag tag;
  union {
    struct StyleBar1_Body_u32 bar1;
    struct StyleBar2_Body_u32 bar2;
    struct StyleBar3_Body_u32 bar3;
  };
};

enum StyleBaz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Baz1,
  Baz2,
  Baz3,
};
#ifndef __cplusplus
typedef uint8_t StyleBaz_Tag;
#endif // __cplusplus

struct StyleBaz1_Body {
  StyleBaz_Tag tag;
  struct StyleBar_u32 _0;
};

struct StyleBaz2_Body {
  StyleBaz_Tag tag;
  struct StylePoint_i32 _0;
};

union StyleBaz {
  StyleBaz_Tag tag;
  struct StyleBaz1_Body baz1;
  struct StyleBaz2_Body baz2;
};

enum StyleTaz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Taz1,
  Taz2,
  Taz3,
};
#ifndef __cplusplus
typedef uint8_t StyleTaz_Tag;
#endif // __cplusplus

struct StyleTaz1_Body {
  struct StyleBar_u32 _0;
};

struct StyleTaz2_Body {
  union StyleBaz _0;
};

struct StyleTaz {
  StyleTaz_Tag tag;
  union {
    struct StyleTaz1_Body taz1;
    struct StyleTaz2_Body taz2;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void foo(const union StyleFoo_i32 *foo,
         const struct StyleBar_i32 *bar,
         const union StyleBaz *baz,
         const struct StyleTaz *taz);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

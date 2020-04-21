#define NOINLINE __attribute__((noinline))
#define NODISCARD [[nodiscard]]


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum FillRule
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  A,
  B,
};
#ifndef __cplusplus
typedef uint8_t FillRule;
#endif // __cplusplus

/**
 * This will have a destructor manually implemented via variant_body, and
 * similarly a Drop impl in Rust.
 */
struct OwnedSlice_u32 {
  uintptr_t len;
  uint32_t *ptr;
};

struct Polygon_u32 {
  FillRule fill;
  struct OwnedSlice_u32 coordinates;
};

/**
 * This will have a destructor manually implemented via variant_body, and
 * similarly a Drop impl in Rust.
 */
struct OwnedSlice_i32 {
  uintptr_t len;
  int32_t *ptr;
};

enum Foo_u32_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Bar_u32,
  Polygon1_u32,
  Slice1_u32,
  Slice2_u32,
  Slice3_u32,
  Slice4_u32,
};
#ifndef __cplusplus
typedef uint8_t Foo_u32_Tag;
#endif // __cplusplus

struct Polygon1_Body_u32 {
  struct Polygon_u32 _0;
};

struct Slice1_Body_u32 {
  struct OwnedSlice_u32 _0;
};

struct Slice2_Body_u32 {
  struct OwnedSlice_i32 _0;
};

struct Slice3_Body_u32 {
  FillRule fill;
  struct OwnedSlice_u32 coords;
};

struct Slice4_Body_u32 {
  FillRule fill;
  struct OwnedSlice_i32 coords;
};

struct Foo_u32 {
  Foo_u32_Tag tag;
  union {
    struct Polygon1_Body_u32 polygon1;
    struct Slice1_Body_u32 slice1;
    struct Slice2_Body_u32 slice2;
    struct Slice3_Body_u32 slice3;
    struct Slice4_Body_u32 slice4;
  };
};

struct Polygon_i32 {
  FillRule fill;
  struct OwnedSlice_i32 coordinates;
};

enum Baz_i32_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Bar2_i32,
  Polygon21_i32,
  Slice21_i32,
  Slice22_i32,
  Slice23_i32,
  Slice24_i32,
};
#ifndef __cplusplus
typedef uint8_t Baz_i32_Tag;
#endif // __cplusplus

struct Polygon21_Body_i32 {
  Baz_i32_Tag tag;
  struct Polygon_i32 _0;
};

struct Slice21_Body_i32 {
  Baz_i32_Tag tag;
  struct OwnedSlice_i32 _0;
};

struct Slice22_Body_i32 {
  Baz_i32_Tag tag;
  struct OwnedSlice_i32 _0;
};

struct Slice23_Body_i32 {
  Baz_i32_Tag tag;
  FillRule fill;
  struct OwnedSlice_i32 coords;
};

struct Slice24_Body_i32 {
  Baz_i32_Tag tag;
  FillRule fill;
  struct OwnedSlice_i32 coords;
};

union Baz_i32 {
  Baz_i32_Tag tag;
  struct Polygon21_Body_i32 polygon21;
  struct Slice21_Body_i32 slice21;
  struct Slice22_Body_i32 slice22;
  struct Slice23_Body_i32 slice23;
  struct Slice24_Body_i32 slice24;
};

enum Taz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Bar3,
  Taz1,
  Taz3,
};
#ifndef __cplusplus
typedef uint8_t Taz_Tag;
#endif // __cplusplus

struct Taz1_Body {
  Taz_Tag tag;
  int32_t _0;
};

struct Taz3_Body {
  Taz_Tag tag;
  struct OwnedSlice_i32 _0;
};

union Taz {
  Taz_Tag tag;
  struct Taz1_Body taz1;
  struct Taz3_Body taz3;
};

enum Tazz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Bar4,
  Taz2,
};
#ifndef __cplusplus
typedef uint8_t Tazz_Tag;
#endif // __cplusplus

struct Taz2_Body {
  Tazz_Tag tag;
  int32_t _0;
};

union Tazz {
  Tazz_Tag tag;
  struct Taz2_Body taz2;
};

enum Tazzz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Bar5,
  Taz5,
};
#ifndef __cplusplus
typedef uint8_t Tazzz_Tag;
#endif // __cplusplus

struct Taz5_Body {
  Tazzz_Tag tag;
  int32_t _0;
};

union Tazzz {
  Tazzz_Tag tag;
  struct Taz5_Body taz5;
};

enum Tazzzz_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Taz6,
  Taz7,
};
#ifndef __cplusplus
typedef uint8_t Tazzzz_Tag;
#endif // __cplusplus

struct Taz6_Body {
  Tazzzz_Tag tag;
  int32_t _0;
};

struct Taz7_Body {
  Tazzzz_Tag tag;
  uint32_t _0;
};

union Tazzzz {
  Tazzzz_Tag tag;
  struct Taz6_Body taz6;
  struct Taz7_Body taz7;
};

enum Qux_Tag
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  Qux1,
  Qux2,
};
#ifndef __cplusplus
typedef uint8_t Qux_Tag;
#endif // __cplusplus

struct Qux1_Body {
  Qux_Tag tag;
  int32_t _0;
};

struct Qux2_Body {
  Qux_Tag tag;
  uint32_t _0;
};

union Qux {
  Qux_Tag tag;
  struct Qux1_Body qux1;
  struct Qux2_Body qux2;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(const struct Foo_u32 *a,
          const union Baz_i32 *b,
          const union Taz *c,
          union Tazz d,
          const union Tazzz *e,
          const union Tazzzz *f,
          const union Qux *g);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

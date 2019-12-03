#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum FillRule {
  A,
  B,
};
typedef uint8_t FillRule;

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

enum Foo_u32_Tag {
  Bar_u32,
  Polygon1_u32,
  Slice1_u32,
  Slice2_u32,
  Slice3_u32,
  Slice4_u32,
};
typedef uint8_t Foo_u32_Tag;

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
  enum Foo_u32_Tag tag;
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

enum Baz_i32_Tag {
  Bar2_i32,
  Polygon21_i32,
  Slice21_i32,
  Slice22_i32,
  Slice23_i32,
  Slice24_i32,
};
typedef uint8_t Baz_i32_Tag;

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
  enum Baz_i32_Tag tag;
  struct Polygon21_Body_i32 polygon21;
  struct Slice21_Body_i32 slice21;
  struct Slice22_Body_i32 slice22;
  struct Slice23_Body_i32 slice23;
  struct Slice24_Body_i32 slice24;
};

enum Taz_Tag {
  Bar3,
  Taz1,
  Taz3,
};
typedef uint8_t Taz_Tag;

struct Taz1_Body {
  Taz_Tag tag;
  int32_t _0;
};

struct Taz3_Body {
  Taz_Tag tag;
  struct OwnedSlice_i32 _0;
};

union Taz {
  enum Taz_Tag tag;
  struct Taz1_Body taz1;
  struct Taz3_Body taz3;
};

enum Tazz_Tag {
  Bar4,
  Taz2,
};
typedef uint8_t Tazz_Tag;

struct Taz2_Body {
  Tazz_Tag tag;
  int32_t _0;
};

union Tazz {
  enum Tazz_Tag tag;
  struct Taz2_Body taz2;
};

void root(const struct Foo_u32 *a, const union Baz_i32 *b, const union Taz *c, union Tazz d);

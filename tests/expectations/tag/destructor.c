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

void root(const struct Foo_u32 *p);

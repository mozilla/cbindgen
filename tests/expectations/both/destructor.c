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
typedef struct OwnedSlice_u32 {
  uintptr_t len;
  uint32_t *ptr;
} OwnedSlice_u32;

typedef struct Polygon_u32 {
  FillRule fill;
  OwnedSlice_u32 coordinates;
} Polygon_u32;

/**
 * This will have a destructor manually implemented via variant_body, and
 * similarly a Drop impl in Rust.
 */
typedef struct OwnedSlice_i32 {
  uintptr_t len;
  int32_t *ptr;
} OwnedSlice_i32;

enum Foo_u32_Tag {
  Bar_u32,
  Polygon1_u32,
  Slice1_u32,
  Slice2_u32,
  Slice3_u32,
  Slice4_u32,
};
typedef uint8_t Foo_u32_Tag;

typedef struct Polygon1_Body_u32 {
  Polygon_u32 _0;
} Polygon1_Body_u32;

typedef struct Slice1_Body_u32 {
  OwnedSlice_u32 _0;
} Slice1_Body_u32;

typedef struct Slice2_Body_u32 {
  OwnedSlice_i32 _0;
} Slice2_Body_u32;

typedef struct Slice3_Body_u32 {
  FillRule fill;
  OwnedSlice_u32 coords;
} Slice3_Body_u32;

typedef struct Slice4_Body_u32 {
  FillRule fill;
  OwnedSlice_i32 coords;
} Slice4_Body_u32;

typedef struct Foo_u32 {
  Foo_u32_Tag tag;
  union {
    Polygon1_Body_u32 polygon1;
    Slice1_Body_u32 slice1;
    Slice2_Body_u32 slice2;
    Slice3_Body_u32 slice3;
    Slice4_Body_u32 slice4;
  };
} Foo_u32;

void root(const Foo_u32 *p);

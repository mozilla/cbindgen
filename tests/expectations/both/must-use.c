#define MUST_USE_FUNC __attribute__((warn_unused_result))
#define MUST_USE_STRUCT __attribute__((warn_unused))
#define MUST_USE_ENUM /* nothing */


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum MaybeOwnedPtr_i32_Tag {
  Owned_i32,
  None_i32,
};
typedef uint8_t MaybeOwnedPtr_i32_Tag;

typedef struct Owned_Body_i32 {
  int32_t *_0;
} Owned_Body_i32;

typedef struct MaybeOwnedPtr_i32 {
  MaybeOwnedPtr_i32_Tag tag;
  union {
    Owned_Body_i32 owned;
  };
} MaybeOwnedPtr_i32;

typedef struct MUST_USE_STRUCT OwnedPtr_i32 {
  int32_t *ptr;
} OwnedPtr_i32;

MUST_USE_FUNC MaybeOwnedPtr_i32 maybe_consume(OwnedPtr_i32 input);

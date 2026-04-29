#define MUST_USE_FUNC __attribute__((warn_unused_result))
#define MUST_USE_STRUCT __attribute__((warn_unused))
#define MUST_USE_ENUM /* nothing */


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum MaybeOwnedPtr_i32_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Owned_i32,
  None_i32,
};
#if __STDC_VERSION__ >= 202311L
typedef enum MaybeOwnedPtr_i32_Tag MaybeOwnedPtr_i32_Tag;
#else
typedef uint8_t MaybeOwnedPtr_i32_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct MUST_USE_STRUCT MaybeOwnedPtr_i32 {
  MaybeOwnedPtr_i32_Tag tag;
  union {
    struct {
      int32_t *owned;
    };
  };
};

struct MUST_USE_STRUCT OwnedPtr_i32 {
  int32_t *ptr;
};

MUST_USE_FUNC struct MaybeOwnedPtr_i32 maybe_consume(struct OwnedPtr_i32 input);

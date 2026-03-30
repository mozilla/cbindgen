#define MUST_USE_FUNC __attribute__((warn_unused_result))
#define MUST_USE_STRUCT __attribute__((warn_unused))
#define MUST_USE_ENUM /* nothing */


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum MaybeOwnedPtr_i32_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Owned_i32,
  None_i32,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum MaybeOwnedPtr_i32_Tag MaybeOwnedPtr_i32_Tag;
#else
typedef uint8_t MaybeOwnedPtr_i32_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct MUST_USE_STRUCT {
  MaybeOwnedPtr_i32_Tag tag;
  union {
    struct {
      int32_t *owned;
    };
  };
} MaybeOwnedPtr_i32;

typedef struct MUST_USE_STRUCT {
  int32_t *ptr;
} OwnedPtr_i32;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

MUST_USE_FUNC MaybeOwnedPtr_i32 maybe_consume(OwnedPtr_i32 input);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

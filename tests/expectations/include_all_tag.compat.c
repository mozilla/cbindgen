#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum UnusedEnum
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A,
  B,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum UnusedEnum UnusedEnum;
#else
typedef uint8_t UnusedEnum;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

struct ExplicitOpaque;

struct OpaqueDependency;

struct UnusedStruct {
  int32_t x;
  float y;
};

typedef uint32_t UnusedTransparent;

union UnusedUnion {
  int32_t x;
  float y;
};

typedef int32_t UnusedAlias;

struct UsesOpaqueDependency {
  struct OpaqueDependency *opaque;
};

struct PublicInPrivateModule {
  int32_t x;
};

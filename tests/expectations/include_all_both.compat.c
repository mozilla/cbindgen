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

typedef struct ExplicitOpaque ExplicitOpaque;

typedef struct OpaqueDependency OpaqueDependency;

typedef struct UnusedStruct {
  int32_t x;
  float y;
} UnusedStruct;

typedef uint32_t UnusedTransparent;

typedef union UnusedUnion {
  int32_t x;
  float y;
} UnusedUnion;

typedef int32_t UnusedAlias;

typedef struct UsesOpaqueDependency {
  struct OpaqueDependency *opaque;
} UsesOpaqueDependency;

typedef struct PublicInPrivateModule {
  int32_t x;
} PublicInPrivateModule;

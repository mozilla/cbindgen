#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum UnusedEnum
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  A,
  B,
};
#if __STDC_VERSION__ >= 202311L
typedef enum UnusedEnum UnusedEnum;
#else
typedef uint8_t UnusedEnum;
#endif // __STDC_VERSION__ >= 202311L

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

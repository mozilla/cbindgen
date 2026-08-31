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

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum StyleOnlyThisShouldBeGenerated
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar,
};
#if __STDC_VERSION__ >= 202311L
typedef enum StyleOnlyThisShouldBeGenerated StyleOnlyThisShouldBeGenerated;
#else
typedef uint8_t StyleOnlyThisShouldBeGenerated;
#endif // __STDC_VERSION__ >= 202311L

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum StyleOnlyThisShouldBeGenerated
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum StyleOnlyThisShouldBeGenerated StyleOnlyThisShouldBeGenerated;
#else
typedef uint8_t StyleOnlyThisShouldBeGenerated;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

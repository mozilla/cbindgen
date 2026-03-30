#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum OnlyThisShouldBeGenerated
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum OnlyThisShouldBeGenerated OnlyThisShouldBeGenerated;
#else
typedef uint8_t OnlyThisShouldBeGenerated;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

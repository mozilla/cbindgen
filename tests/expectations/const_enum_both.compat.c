#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum FillRule
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Nonzero,
  Evenodd,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum FillRule FillRule;
#else
typedef uint8_t FillRule;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct Style {
  FillRule rule;
} Style;
#define Style_DEFAULT_RULE Nonzero
#define Style_ALL_RULES { Nonzero, Evenodd, }

#define DEFAULT_FILL_RULE Nonzero

#define ALL_FILL_RULES { Nonzero, Evenodd, }

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(FillRule rule, struct Style style);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

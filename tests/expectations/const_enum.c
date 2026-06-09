#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum FillRule
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Nonzero,
  Evenodd,
};
#if __STDC_VERSION__ >= 202311L
typedef enum FillRule FillRule;
#else
typedef uint8_t FillRule;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  FillRule rule;
} Style;
#define Style_DEFAULT_RULE Nonzero
#define Style_ALL_RULES { Nonzero, Evenodd, }

#define DEFAULT_FILL_RULE Nonzero

#define ALL_FILL_RULES { Nonzero, Evenodd, }

void root(FillRule rule, Style style);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Regression test: in C++ output, the generated static variant constructors of a
 * tagged enum must value-initialize their `result` local (`Foo result{};`) so that
 * the payload union is zero-initialized rather than left indeterminate. This avoids
 * false "uninitialized scalar" reports from static analysis for no-payload variants.
 *
 */
enum ValueInitEnum_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  /**
   * No payload: `result` is returned with only the tag set, so the union must be
   * value-initialized.
   */
  Empty,
  /**
   * Payload variant: constructed via placement new over the value-initialized union.
   */
  Number,
  Pair,
};
#if __STDC_VERSION__ >= 202311L
typedef enum ValueInitEnum_Tag ValueInitEnum_Tag;
#else
typedef uint8_t ValueInitEnum_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  int32_t _0;
  uint64_t _1;
} Pair_Body;

typedef struct {
  ValueInitEnum_Tag tag;
  union {
    struct {
      int32_t number;
    };
    Pair_Body pair;
  };
} ValueInitEnum;

void root(ValueInitEnum e);

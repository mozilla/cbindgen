#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FOO 10

#define DELIMITER ':'

#define LEFTCURLY '{'

#define QUOTE '\''

#define TAB '\t'

#define NEWLINE '\n'

#define HEART U'\U00002764'

#define EQUID U'\U00010083'

#define ZOM 3.14

/**
 * A single-line doc comment.
 */
#define POS_ONE 1

/**
 * A
 * multi-line
 * doc
 * comment.
 */
#define NEG_ONE -1

#define SHIFT 3

#define XBOOL 1

#define XFALSE1 ((0 << 3) | 1)

#define XTRUE1 (1 << (3 | 1))

#define XFALSE2 (0 << SHIFT)

#define CAST (uint8_t)'A'

#define DOUBLE_CAST (uint32_t)(float)1

typedef struct Foo {
  int32_t x[FOO];
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

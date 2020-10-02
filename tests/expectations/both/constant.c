#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define CAST (uint8_t)'A'

#define DELIMITER ':'

#define DOUBLE_CAST (uint32_t)(float)1

#define EQUID U'\U00010083'

#define FOO 10

#define HEART U'\U00002764'

#define LEFTCURLY '{'

/**
 * A
 * multi-line
 * doc
 * comment.
 */
#define NEG_ONE -1

#define NEWLINE '\n'

/**
 * A single-line doc comment.
 */
#define POS_ONE 1

#define QUOTE '\''

#define SHIFT 3

#define TAB '\t'

#define XBOOL 1

#define XFALSE ((0 << SHIFT) | XBOOL)

#define XTRUE (1 << (SHIFT | XBOOL))

#define ZOM 3.14

typedef struct Foo {
  int32_t x[FOO];
} Foo;

void root(Foo x);

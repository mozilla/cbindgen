#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define DELIMITER ':'

#define EQUID L'\U00010083'

#define FOO 10

#define HEART L'\U00002764'

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

typedef struct {
  int32_t x[FOO];
} Foo;

void root(Foo x);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define DELIMITER ':'

#define FOO 10

#define HEART L'\u2764'

#define LEFTCURLY '{'

#define NEWLINE '\n'

#define QUOTE '\''

#define TAB '\t'

#define ZOM 3.14

typedef struct {
  int32_t x[FOO];
} Foo;

#define BAR "hello world"

void root(Foo x);

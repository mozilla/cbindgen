#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define DELIMITER ':'

#define FOO 10

#define HEART '\u2764'

#define NEWLINE '\n'

#define QUOTE '\''

#define TAB '\t'

#define ZOM 3.14

struct Foo {
  int32_t x[FOO];
};

void root(struct Foo x);

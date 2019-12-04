#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define DELIMITER ':'

#define EQUID L'\u10083'

#define FOO 10

#define HEART L'\u2764'

#define LEFTCURLY '{'

#define NEG_ONE -1

#define NEWLINE '\n'

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Foo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

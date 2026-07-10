#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define STRING "string"

#define C_STRING "c string"

#define EMPTY ""

#define WITH_QUOTES "a \"quoted\" word"

#define WITH_BACKSLASH "back\\slash"

#define WITH_CONTROLS "tab\tnewline\nreturn\r"

#define OTHER_CONTROLS "\x01\x0b\x0c\x7f"

#define TRIGRAPHS "what\?\?!"

#define INTERIOR_NUL "a\x00" "b"

#define NON_UTF8 "\xff\xfe"

#define NON_ASCII "caf\xc3\xa9"

#define HEX_ADJACENT "\xc3" "a"

#define HEX_ADJACENT_UPPER "\xc3" "F"

#define HEX_NON_ADJACENT "\xc3z"

#define ARRAY { "first", "second", }

/**
 * A documented string constant.
 */
#define DOCUMENTED "documented"

void root(void);

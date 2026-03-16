#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 *With doc attr, each attr contribute to one line of document
 *like this one with a new line character at its end
 *and this one as well. So they are in the same paragraph
 *
 *We treat empty doc comments as empty lines, so they break to the next paragraph.
 *
 * Newlines are preserved with leading spaces added
 * to prettify and avoid misinterpreting leading symbols.
 *like headings and lists.
 *
 * Line ends with two new lines
 *
 * Should break to next paragraph
 */
void root(void);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

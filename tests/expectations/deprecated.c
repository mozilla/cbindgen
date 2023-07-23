#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum DeprecatedEnum {
  A = 0,
};
typedef int32_t DeprecatedEnum;

#ifdef __cplusplus
[[deprecated]]
#endif // __cplusplus
void deprecated_without_note(void);

#ifdef __cplusplus
[[deprecated("This is a note")]]
#endif // __cplusplus
void deprecated_without_bracket(void);

#ifdef __cplusplus
[[deprecated("This is a note")]]
#endif // __cplusplus
void deprecated_with_note(void);

#ifdef __cplusplus
[[deprecated("This is a note")]]
#endif // __cplusplus
void deprecated_with_note_and_since(void);

#ifdef __cplusplus
[[deprecated("This quote \" requires to be quoted, and this [\n] requires to be escaped")]]
#endif // __cplusplus
void deprecated_with_note_which_requires_to_be_escaped(void);

void dummy(DeprecatedEnum a);

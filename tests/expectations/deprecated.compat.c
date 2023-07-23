#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum DeprecatedEnum
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
  A = 0,
};
#ifndef __cplusplus
typedef int32_t DeprecatedEnum;
#endif // __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

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

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

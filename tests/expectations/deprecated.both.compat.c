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

typedef struct DeprecatedStruct {
  int32_t a;
} DeprecatedStruct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if __STDC_VERSION__ >= 202311L
[[deprecated]]
#endif // __STDC_VERSION__ >= 202311L
void deprecated_without_note(void);

#if __STDC_VERSION__ >= 202311L
[[deprecated("This is a note")]]
#endif // __STDC_VERSION__ >= 202311L
void deprecated_without_bracket(void);

#if __STDC_VERSION__ >= 202311L
[[deprecated("This is a note")]]
#endif // __STDC_VERSION__ >= 202311L
void deprecated_with_note(void);

#if __STDC_VERSION__ >= 202311L
[[deprecated("This is a note")]]
#endif // __STDC_VERSION__ >= 202311L
void deprecated_with_note_and_since(void);

#if __STDC_VERSION__ >= 202311L
[[deprecated("This quote \" requires to be quoted, and this [\n] requires to be escaped")]]
#endif // __STDC_VERSION__ >= 202311L
void deprecated_with_note_which_requires_to_be_escaped(void);

void dummy(DeprecatedEnum a, struct DeprecatedStruct b);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum DEPRECATED_ENUM DeprecatedEnum
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
  A = 0,
};
#ifndef __cplusplus
typedef int32_t DeprecatedEnum;
#endif // __cplusplus

enum DEPRECATED_ENUM_WITH_NOTE("This is a note") DeprecatedEnumWithNote
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
  B = 0,
};
#ifndef __cplusplus
typedef int32_t DeprecatedEnumWithNote;
#endif // __cplusplus

typedef struct DEPRECATED_STRUCT DeprecatedStruct {
  int32_t a;
} DeprecatedStruct;

typedef struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") DeprecatedStructWithNote {
  int32_t a;
} DeprecatedStructWithNote;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

DEPRECATED_FUNC void deprecated_without_note(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_without_bracket(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note(void);

DEPRECATED_FUNC_WITH_NOTE("This is a note") void deprecated_with_note_and_since(void);

DEPRECATED_FUNC_WITH_NOTE("This quote \" requires to be quoted, and this [\n] requires to be escaped")
void deprecated_with_note_which_requires_to_be_escaped(void);

void dummy(DeprecatedEnum a,
           DeprecatedEnumWithNote b,
           struct DeprecatedStruct c,
           struct DeprecatedStructWithNote d);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

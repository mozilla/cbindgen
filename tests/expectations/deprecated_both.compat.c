#define DEPRECATED_FUNC __attribute__((deprecated))
#define DEPRECATED_STRUCT __attribute__((deprecated))
#define DEPRECATED_ENUM __attribute__((deprecated))
#define DEPRECATED_ENUM_VARIANT __attribute__((deprecated))
#define DEPRECATED_FUNC_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_STRUCT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))
#define DEPRECATED_ENUM_VARIANT_WITH_NOTE(...) __attribute__((deprecated(__VA_ARGS__)))


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum DEPRECATED_ENUM DeprecatedEnum
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum DeprecatedEnum DeprecatedEnum;
#else
typedef int32_t DeprecatedEnum;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum DEPRECATED_ENUM_WITH_NOTE("This is a note") DeprecatedEnumWithNote
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  B = 0,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum DeprecatedEnumWithNote DeprecatedEnumWithNote;
#else
typedef int32_t DeprecatedEnumWithNote;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum EnumWithDeprecatedVariants
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : int32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  C = 0,
  D DEPRECATED_ENUM_VARIANT = 1,
  E DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 2,
  F DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 3,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum EnumWithDeprecatedVariants EnumWithDeprecatedVariants;
#else
typedef int32_t EnumWithDeprecatedVariants;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct DEPRECATED_STRUCT DeprecatedStruct {
  int32_t a;
} DeprecatedStruct;

typedef struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") DeprecatedStructWithNote {
  int32_t a;
} DeprecatedStructWithNote;

enum EnumWithDeprecatedStructVariants_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar DEPRECATED_ENUM_VARIANT,
  Baz DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note"),
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum EnumWithDeprecatedStructVariants_Tag EnumWithDeprecatedStructVariants_Tag;
#else
typedef uint8_t EnumWithDeprecatedStructVariants_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct DEPRECATED_STRUCT Bar_Body {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  int16_t y;
} Bar_Body;

typedef struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") Baz_Body {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  uint8_t y;
} Baz_Body;

typedef union EnumWithDeprecatedStructVariants {
  EnumWithDeprecatedStructVariants_Tag tag;
  struct {
    EnumWithDeprecatedStructVariants_Tag foo_tag;
    int16_t foo;
  };
  Bar_Body bar;
  Baz_Body baz;
} EnumWithDeprecatedStructVariants;

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
           EnumWithDeprecatedVariants c,
           struct DeprecatedStruct d,
           struct DeprecatedStructWithNote e,
           union EnumWithDeprecatedStructVariants f);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

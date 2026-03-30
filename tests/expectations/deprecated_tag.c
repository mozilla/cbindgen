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
#if __STDC_VERSION__ >= 202311L
  : int32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  A = 0,
};
#if __STDC_VERSION__ >= 202311L
typedef enum DeprecatedEnum DeprecatedEnum;
#else
typedef int32_t DeprecatedEnum;
#endif // __STDC_VERSION__ >= 202311L

enum DEPRECATED_ENUM_WITH_NOTE("This is a note") DeprecatedEnumWithNote
#if __STDC_VERSION__ >= 202311L
  : int32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  B = 0,
};
#if __STDC_VERSION__ >= 202311L
typedef enum DeprecatedEnumWithNote DeprecatedEnumWithNote;
#else
typedef int32_t DeprecatedEnumWithNote;
#endif // __STDC_VERSION__ >= 202311L

enum EnumWithDeprecatedVariants
#if __STDC_VERSION__ >= 202311L
  : int32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  C = 0,
  D DEPRECATED_ENUM_VARIANT = 1,
  E DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 2,
  F DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note") = 3,
};
#if __STDC_VERSION__ >= 202311L
typedef enum EnumWithDeprecatedVariants EnumWithDeprecatedVariants;
#else
typedef int32_t EnumWithDeprecatedVariants;
#endif // __STDC_VERSION__ >= 202311L

struct DEPRECATED_STRUCT DeprecatedStruct {
  int32_t a;
};

struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") DeprecatedStructWithNote {
  int32_t a;
};

enum EnumWithDeprecatedStructVariants_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Foo,
  Bar DEPRECATED_ENUM_VARIANT,
  Baz DEPRECATED_ENUM_VARIANT_WITH_NOTE("This is a note"),
};
#if __STDC_VERSION__ >= 202311L
typedef enum EnumWithDeprecatedStructVariants_Tag EnumWithDeprecatedStructVariants_Tag;
#else
typedef uint8_t EnumWithDeprecatedStructVariants_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct DEPRECATED_STRUCT Bar_Body {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  int16_t y;
};

struct DEPRECATED_STRUCT_WITH_NOTE("This is a note") Baz_Body {
  EnumWithDeprecatedStructVariants_Tag tag;
  uint8_t x;
  uint8_t y;
};

union EnumWithDeprecatedStructVariants {
  EnumWithDeprecatedStructVariants_Tag tag;
  struct {
    EnumWithDeprecatedStructVariants_Tag foo_tag;
    int16_t foo;
  };
  struct Bar_Body bar;
  struct Baz_Body baz;
};

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

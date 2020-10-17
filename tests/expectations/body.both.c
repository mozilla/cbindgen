#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
} MyCLikeEnum;

typedef enum MyCLikeEnum_Prepended {
  Foo1_Prepended,
  Bar1_Prepended,
  Baz1_Prepended,
} MyCLikeEnum_Prepended;

typedef struct MyFancyStruct {
  int32_t i;
#ifdef __cplusplus
  inline void foo();
#endif
} MyFancyStruct;

typedef enum MyFancyEnum_Tag {
  Foo,
  Bar,
  Baz,
} MyFancyEnum_Tag;

typedef struct Bar_Body {
  int32_t _0;
} Bar_Body;

typedef struct Baz_Body {
  int32_t _0;
} Baz_Body;

typedef struct MyFancyEnum {
  MyFancyEnum_Tag tag;
  union {
    Bar_Body bar;
    Baz_Body baz;
  };
#ifdef __cplusplus
  inline void wohoo();
#endif
} MyFancyEnum;

typedef union MyUnion {
  float f;
  uint32_t u;
  int32_t extra_member; // yolo
} MyUnion;

typedef struct MyFancyStruct_Prepended {
#ifdef __cplusplus
  inline void prepended_wohoo();
#endif
  int32_t i;
} MyFancyStruct_Prepended;

typedef enum MyFancyEnum_Prepended_Tag {
  Foo_Prepended,
  Bar_Prepended,
  Baz_Prepended,
} MyFancyEnum_Prepended_Tag;

typedef struct Bar_Prepended_Body {
  int32_t _0;
} Bar_Prepended_Body;

typedef struct Baz_Prepended_Body {
  int32_t _0;
} Baz_Prepended_Body;

typedef struct MyFancyEnum_Prepended {
  #ifdef __cplusplus
    inline void wohoo();
  #endif
  MyFancyEnum_Prepended_Tag tag;
  union {
    Bar_Prepended_Body bar_prepended;
    Baz_Prepended_Body baz_prepended;
  };
} MyFancyEnum_Prepended;

typedef union MyUnion_Prepended {
  int32_t extra_member; // yolo
  float f;
  uint32_t u;
} MyUnion_Prepended;

void root(MyFancyStruct s,
          MyFancyEnum e,
          MyCLikeEnum c,
          MyUnion u,
          MyFancyStruct_Prepended sp,
          MyFancyEnum_Prepended ep,
          MyCLikeEnum_Prepended cp,
          MyUnion_Prepended up);

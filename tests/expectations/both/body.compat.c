#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

typedef enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
} MyCLikeEnum;

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(MyFancyStruct s, MyFancyEnum e, MyCLikeEnum c, MyUnion u);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

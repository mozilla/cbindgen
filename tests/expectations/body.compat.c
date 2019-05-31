#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Foo1,
  Bar1,
  Baz1,
} MyCLikeEnum;
#ifndef __cplusplus

#endif // __cplusplus


typedef struct {
  int32_t i;
#ifdef __cplusplus
  inline void foo();
#endif
} MyFancyStruct;

typedef enum {
  Foo,
  Bar,
  Baz,
} MyFancyEnum_Tag;
#ifndef __cplusplus

#endif // __cplusplus


typedef struct {
  int32_t _0;
} Bar_Body;

typedef struct {
  int32_t _0;
} Baz_Body;

typedef struct {
  MyFancyEnum_Tag tag;
  union {
    Bar_Body bar;
    Baz_Body baz;
  };
#ifdef __cplusplus
  inline void wohoo();
#endif
} MyFancyEnum;

typedef union {
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

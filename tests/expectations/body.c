#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

typedef enum {
  Foo1,
  Bar1,
  Baz1,
} MyCLikeEnum;

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

void root(MyFancyStruct s, MyFancyEnum e, MyCLikeEnum c, MyUnion u);

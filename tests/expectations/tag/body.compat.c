#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
};
#ifndef __cplusplus

#endif // __cplusplus


struct MyFancyStruct {
  int32_t i;
#ifdef __cplusplus
  inline void foo();
#endif
};

enum MyFancyEnum_Tag {
  Foo,
  Bar,
  Baz,
};
#ifndef __cplusplus

#endif // __cplusplus


struct Bar_Body {
  int32_t _0;
};

struct Baz_Body {
  int32_t _0;
};

struct MyFancyEnum {
  enum MyFancyEnum_Tag tag;
  union {
    struct Bar_Body bar;
    struct Baz_Body baz;
  };
#ifdef __cplusplus
  inline void wohoo();
#endif
};

union MyUnion {
  float f;
  uint32_t u;
  int32_t extra_member; // yolo
};

#ifdef __cplusplus

extern "C" {

#endif // __cplusplus

void root(struct MyFancyStruct s, struct MyFancyEnum e, enum MyCLikeEnum c, union MyUnion u);

#ifdef __cplusplus

} // extern "C"

#endif // __cplusplus

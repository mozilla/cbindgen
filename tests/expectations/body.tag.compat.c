#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
};

enum MyCLikeEnum_Prepended {
  Foo1_Prepended,
  Bar1_Prepended,
  Baz1_Prepended,
};

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
    int32_t extra_member;
};

struct MyFancyStruct_Prepended {
#ifdef __cplusplus
  inline void prepended_wohoo();
#endif
  int32_t i;
};

enum MyFancyEnum_Prepended_Tag {
  Foo_Prepended,
  Bar_Prepended,
  Baz_Prepended,
};

struct Bar_Prepended_Body {
  int32_t _0;
};

struct Baz_Prepended_Body {
  int32_t _0;
};

struct MyFancyEnum_Prepended {
  #ifdef __cplusplus
    inline void wohoo();
  #endif
  enum MyFancyEnum_Prepended_Tag tag;
  union {
    struct Bar_Prepended_Body bar_prepended;
    struct Baz_Prepended_Body baz_prepended;
  };
};

union MyUnion_Prepended {
    int32_t extra_member;
  float f;
  uint32_t u;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct MyFancyStruct s,
          struct MyFancyEnum e,
          enum MyCLikeEnum c,
          union MyUnion u,
          struct MyFancyStruct_Prepended sp,
          struct MyFancyEnum_Prepended ep,
          enum MyCLikeEnum_Prepended cp,
          union MyUnion_Prepended up);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
};

enum class MyCLikeEnum_Prepended {
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

struct MyFancyEnum {
  enum class Tag {
    Foo,
    Bar,
    Baz,
  };

  Tag tag;
  union {
    struct {
      int32_t bar;
    };
    struct {
      int32_t baz;
    };
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

struct MyFancyEnum_Prepended {
#ifdef __cplusplus
    inline void wohoo();
#endif
  enum class Tag {
    Foo_Prepended,
    Bar_Prepended,
    Baz_Prepended,
  };

  Tag tag;
  union {
    struct {
      int32_t bar_prepended;
    };
    struct {
      int32_t baz_prepended;
    };
  };
};

union MyUnion_Prepended {
    int32_t extra_member;
  float f;
  uint32_t u;
};

extern "C" {

void root(MyFancyStruct s,
          MyFancyEnum e,
          MyCLikeEnum c,
          MyUnion u,
          MyFancyStruct_Prepended sp,
          MyFancyEnum_Prepended ep,
          MyCLikeEnum_Prepended cp,
          MyUnion_Prepended up);

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>

enum class MyCLikeEnum {
  Foo1,
  Bar1,
  Baz1,
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

  struct Bar_Body {
    int32_t _0;
  };

  struct Baz_Body {
    int32_t _0;
  };

  Tag tag;
  union {
    Bar_Body bar;
    Baz_Body baz;
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

extern "C" {

void root(MyFancyStruct s, MyFancyEnum e, MyCLikeEnum c, MyUnion u);

} // extern "C"

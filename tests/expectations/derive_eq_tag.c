#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Foo {
  bool a;
  int32_t b;
};

enum Bar_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Baz,
  Bazz,
  FooNamed,
  FooParen,
};
#if __STDC_VERSION__ >= 202311L
typedef enum Bar_Tag Bar_Tag;
#else
typedef uint8_t Bar_Tag;
#endif // __STDC_VERSION__ >= 202311L

struct Bazz_Body {
  Bar_Tag tag;
  struct Foo named;
};

struct FooNamed_Body {
  Bar_Tag tag;
  int32_t different;
  uint32_t fields;
};

struct FooParen_Body {
  Bar_Tag tag;
  int32_t _0;
  struct Foo _1;
};

union Bar {
  Bar_Tag tag;
  struct Bazz_Body bazz;
  struct FooNamed_Body foo_named;
  struct FooParen_Body foo_paren;
};

struct Foo root(union Bar aBar);

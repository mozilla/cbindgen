#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Foo {
  int32_t x;
} Foo;

typedef struct RenamedTy {
  uint64_t y;
} RenamedTy;

#if !defined(DEFINE_FREEBSD)
typedef struct NoExternTy {
  uint8_t field;
} NoExternTy;
#endif

#if !defined(DEFINE_FREEBSD)
typedef struct ContainsNoExternTy {
  NoExternTy field;
} ContainsNoExternTy;
#endif

#if defined(DEFINE_FREEBSD)
typedef struct ContainsNoExternTy {
  uint64_t field;
} ContainsNoExternTy;
#endif

void root(Foo a);

void renamed_func(RenamedTy a);

void no_extern_func(ContainsNoExternTy a);

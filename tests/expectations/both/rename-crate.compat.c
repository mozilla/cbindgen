#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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

typedef struct RenamedTy {
  uint64_t y;
} RenamedTy;

typedef struct Foo {
  int32_t x;
} Foo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void no_extern_func(ContainsNoExternTy a);

void renamed_func(RenamedTy a);

void root(Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

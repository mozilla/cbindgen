#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if !defined(DEFINE_FREEBSD)
struct NoExternTy {
  uint8_t field;
};
#endif

#if !defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  struct NoExternTy field;
};
#endif

#if defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  uint64_t field;
};
#endif

struct RenamedTy {
  uint64_t y;
};

struct Foo {
  int32_t x;
};

#ifdef __cplusplus

extern "C" {

#endif // __cplusplus

void no_extern_func(struct ContainsNoExternTy a);

void renamed_func(struct RenamedTy a);

void root(struct Foo a);

#ifdef __cplusplus

} // extern "C"

#endif // __cplusplus

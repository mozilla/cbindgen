#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if !defined(DEFINE_FREEBSD)
typedef struct {
  uint8_t field;
} NoExternTy;
#endif

#if !defined(DEFINE_FREEBSD)
typedef struct {
  NoExternTy field;
} ContainsNoExternTy;
#endif

#if defined(DEFINE_FREEBSD)
typedef struct {
  uint64_t field;
} ContainsNoExternTy;
#endif

typedef struct {
  uint64_t y;
} RenamedTy;

typedef struct {
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

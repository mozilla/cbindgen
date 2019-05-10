#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

#if !defined(DEFINE_FREEBSD)
struct NoExternTy {
  uint8_t field;
};
#endif

#if !defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  NoExternTy field;
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

extern "C" {

void no_extern_func(ContainsNoExternTy a);

void renamed_func(RenamedTy a);

void root(Foo a);

} // extern "C"

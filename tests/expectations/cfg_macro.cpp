#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#if (defined(A) && defined(B_C))
#define __CBINDGEN_CFG_A2_D1_A_D3_B_C(...) __VA_ARGS__
#else
#define __CBINDGEN_CFG_A2_D1_A_D3_B_C(...)
#endif
#if (defined(A_B) && defined(C))
#define __CBINDGEN_CFG_A2_D3_A_B_D1_C(...) __VA_ARGS__
#else
#define __CBINDGEN_CFG_A2_D3_A_B_D1_C(...)
#endif

struct Collision {
#if (defined(A) && defined(B_C))
  int32_t x
#endif
  ;
#if (defined(A_B) && defined(C))
  int32_t y
#endif
  ;
};

struct Nested {
  Collision collision;
  const uint8_t *pointer;
};

struct Foo {
#if defined(PLATFORM_WIN)
  int32_t x
#endif
  ;
};

#define NESTED Nested{ /* .collision = */ Collision{ __CBINDGEN_CFG_A2_D1_A_D3_B_C(/* .x = */ 1,) __CBINDGEN_CFG_A2_D3_A_B_D1_C(/* .y = */ 2,) }, /* .pointer = */ (const uint8_t*)0 }

constexpr const Foo FOO = Foo{
#if defined(PLATFORM_WIN)
  /* .x = */ 0
#endif
};

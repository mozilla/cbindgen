#include <cstdint>
#include <cstdlib>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum class BarType : uint32_t {
  A = 0,
  B = 1,
  C = 2,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum class FooType : uint32_t {
  A = 0,
  B = 1,
  C = 2,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
struct FooHandle {
  FooType ty;
  int32_t x;
  float y;
};
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
};
#endif

extern "C" {

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a);
#endif

} // extern "C"

#include <cstdarg>
#include <cstdint>
#include <cstdlib>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum class BarType : uint32_t {
  A,
  B,
  C,
};
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum class FooType : uint32_t {
  A,
  B,
  C,
};
#endif

struct ConditionalField {
  #if defined(X11)
  int32_t field
  #endif
  ;
};

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

void cond(ConditionalField a);

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a);
#endif

} // extern "C"

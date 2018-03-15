#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum BarType {
  A,
  B,
  C,
};
typedef uint32_t BarType;
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum FooType {
  A,
  B,
  C,
};
typedef uint32_t FooType;
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

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(struct FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(struct BarHandle a);
#endif

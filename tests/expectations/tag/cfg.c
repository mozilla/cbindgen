#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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

void cond(struct ConditionalField a);

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(struct FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(struct BarHandle a);
#endif

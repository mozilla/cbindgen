#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum BarType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  A,
  B,
  C,
};
#ifndef __cplusplus

typedef uint32_t BarType;
#endif // __cplusplus

#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum FooType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  A,
  B,
  C,
};
#ifndef __cplusplus

typedef uint32_t FooType;
#endif // __cplusplus

#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
typedef struct FooHandle {
  FooType ty;
  int32_t x;
  float y;
} FooHandle;
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
typedef struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
} BarHandle;
#endif

#ifdef __cplusplus

extern "C" {

#endif // __cplusplus

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a);
#endif

#ifdef __cplusplus

} // extern "C"

#endif // __cplusplus

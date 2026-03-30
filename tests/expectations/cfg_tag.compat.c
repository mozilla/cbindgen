#if 0
DEF PLATFORM_UNIX = 0
DEF PLATFORM_WIN = 0
DEF X11 = 0
DEF M_32 = 0
#endif
#define PLATFORM_UNIX 1


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if (defined(PLATFORM_UNIX) && defined(X11))
enum FooType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A,
  B,
  C,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum FooType FooType;
#else
typedef uint32_t FooType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
enum BarType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  A,
  B,
  C,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum BarType BarType;
#else
typedef uint32_t BarType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus
#endif

struct Flags {
  uint8_t _0;
};
/**
 * none
 */
#define Flags_NONE (Flags){ ._0 = (uint8_t)0 }
#if defined(PLATFORM_WIN)
#define Flags_A (Flags){ ._0 = (uint8_t)(1 << 0) }
#endif
#if defined(PLATFORM_UNIX)
#define Flags_A (Flags){ ._0 = (uint8_t)(1 << 1) }
#endif
#if defined(PLATFORM_WIN)
#define Flags_B (Flags){ ._0 = (uint8_t)((Flags_A)._0 | (1 << 3)) }
#endif
#if defined(PLATFORM_UNIX)
#define Flags_B (Flags){ ._0 = (uint8_t)((Flags_A)._0 | (1 << 4)) }
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
struct FooHandle {
  FooType ty;
  struct Flags flags;
  int32_t x;
  float y;
};
#endif

enum C_Tag
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  C1,
  C2,
#if defined(PLATFORM_WIN)
  C3,
#endif
#if defined(PLATFORM_UNIX)
  C5,
#endif
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum C_Tag C_Tag;
#else
typedef uint8_t C_Tag;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

#if defined(PLATFORM_UNIX)
struct C5_Body {
  C_Tag tag;
  int32_t int_;
};
#endif

union C {
  C_Tag tag;
#if defined(PLATFORM_UNIX)
  struct C5_Body c5;
#endif
};

#if (defined(PLATFORM_WIN) || defined(M_32))
struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
};
#endif

struct ConditionalField {
#if defined(X11)
  int32_t field
#endif
  ;
};
#define ConditionalField_ZERO (ConditionalField){ .field = 0 }
#define ConditionalField_ONE (ConditionalField){ .field = 1 }

struct Normal {
  int32_t x;
  float y;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if defined(PLATFORM_WIN)
extern int32_t global_array_with_different_sizes[2];
#endif

#if defined(PLATFORM_UNIX)
extern int32_t global_array_with_different_sizes[1];
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(struct FooHandle a, union C c);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(struct BarHandle a, union C c);
#endif

void cond(struct ConditionalField a);

#if defined(PLATFORM_WIN)
extern int32_t foo(void);
#endif

#if defined(PLATFORM_WIN)
extern void bar(struct Normal a);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

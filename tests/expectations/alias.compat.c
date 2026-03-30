#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Status
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Ok,
  Err,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Status Status;
#else
typedef uint32_t Status;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  int32_t a;
  float b;
} Dep;

typedef struct {
  int32_t a;
  int32_t b;
  Dep c;
} Foo_i32;

typedef Foo_i32 IntFoo;

typedef struct {
  double a;
  double b;
  Dep c;
} Foo_f64;

typedef Foo_f64 DoubleFoo;

typedef int32_t Unit;

typedef Status SpecialStatus;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

enum Status {
  Ok,
  Err,
};
typedef uint32_t Status;

typedef struct Dep {
  int32_t a;
  float b;
} Dep;

typedef struct Foo_i32 {
  int32_t a;
  int32_t b;
  Dep c;
} Foo_i32;

typedef Foo_i32 IntFoo;

typedef struct Foo_f64 {
  double a;
  double b;
  Dep c;
} Foo_f64;

typedef Foo_f64 DoubleFoo;

typedef int32_t Unit;

typedef Status SpecialStatus;

void root(IntFoo x, DoubleFoo y, Unit z, SpecialStatus w);

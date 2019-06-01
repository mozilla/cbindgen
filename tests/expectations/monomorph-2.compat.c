#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A A;

typedef struct B B;

typedef struct {
  B *members;
  uintptr_t count;
} List_B;

typedef struct {
  A *members;
  uintptr_t count;
} List_A;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void bar(List_B b);

void foo(List_A a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

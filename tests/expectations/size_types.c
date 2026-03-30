#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum UE
#if __STDC_VERSION__ >= 202311L
  : size_t
#endif // __STDC_VERSION__ >= 202311L
 {
  UV,
};
#if __STDC_VERSION__ >= 202311L
typedef enum UE UE;
#else
typedef size_t UE;
#endif // __STDC_VERSION__ >= 202311L

enum IE
#if __STDC_VERSION__ >= 202311L
  : ptrdiff_t
#endif // __STDC_VERSION__ >= 202311L
 {
  IV,
};
#if __STDC_VERSION__ >= 202311L
typedef enum IE IE;
#else
typedef ptrdiff_t IE;
#endif // __STDC_VERSION__ >= 202311L

typedef size_t Usize;

typedef ptrdiff_t Isize;

void root(Usize, Isize, UE, IE);

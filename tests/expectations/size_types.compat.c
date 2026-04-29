#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum UE
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : size_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  UV,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum UE UE;
#else
typedef size_t UE;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

enum IE
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : ptrdiff_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  IV,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum IE IE;
#else
typedef ptrdiff_t IE;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef size_t Usize;

typedef ptrdiff_t Isize;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(Usize, Isize, UE, IE);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

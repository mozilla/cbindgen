#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Enum
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  a,
  b,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum Enum Enum;
#else
typedef uint8_t Enum;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct {
  Enum field;
} Struct;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const Enum STATIC;

void fn(Struct arg);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum BindingType
#if defined(__cplusplus) || __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // defined(__cplusplus) || __STDC_VERSION__ >= 202311L
 {
  Buffer = 0,
  NotBuffer = 1,
};
#ifndef __cplusplus
#if __STDC_VERSION__ >= 202311L
typedef enum BindingType BindingType;
#else
typedef uint32_t BindingType;
#endif // __STDC_VERSION__ >= 202311L
#endif // __cplusplus

typedef struct BindGroupLayoutEntry {
  BindingType ty;
} BindGroupLayoutEntry;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct BindGroupLayoutEntry entry);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

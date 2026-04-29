#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum BindingType
#if __STDC_VERSION__ >= 202311L
  : uint32_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Buffer = 0,
  NotBuffer = 1,
};
#if __STDC_VERSION__ >= 202311L
typedef enum BindingType BindingType;
#else
typedef uint32_t BindingType;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  BindingType ty;
} BindGroupLayoutEntry;

void root(BindGroupLayoutEntry entry);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define capi_constant_abc 10

enum capi_enumeration
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  capi_enumeration_x = 0,
  capi_enumeration_y = 1,
};
#ifndef __cplusplus
typedef uint8_t capi_enumeration;
#endif // __cplusplus

typedef struct capi_struct_abc capi_struct_abc;

typedef struct capi_union_ghi capi_union_ghi;

typedef struct capi_struct_def {
  int32_t x;
  float y;
} capi_struct_def;

typedef union capi_union_jkl {
  int32_t x;
  float y;
} capi_union_jkl;

typedef struct capi_struct_abc capi_type_alias;

#define capi_constant_expression (intptr_t)(capi_type_alias*)10

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const int32_t StaticAbc;

void root(const struct capi_struct_abc *a,
          struct capi_struct_def b,
          struct capi_union_ghi c,
          union capi_union_jkl d,
          capi_enumeration e,
          capi_type_alias f);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

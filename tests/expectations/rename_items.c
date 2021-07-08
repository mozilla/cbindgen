#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define capi_constant_abc 10

enum capi_enumeration {
  capi_enumeration_x = 0,
  capi_enumeration_y = 1,
};
typedef uint8_t capi_enumeration;

typedef struct capi_struct_abc capi_struct_abc;

typedef struct capi_union_ghi capi_union_ghi;

typedef struct {
  int32_t x;
  float y;
} capi_struct_def;

typedef union {
  int32_t x;
  float y;
} capi_union_jkl;

typedef capi_struct_abc capi_type_alias;

#define capi_constant_expression (intptr_t)(capi_type_alias*)10

extern const int32_t StaticAbc;

void root(const capi_struct_abc *a,
          capi_struct_def b,
          capi_union_ghi c,
          capi_union_jkl d,
          capi_enumeration e,
          capi_type_alias f);

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const int32_t capi_constant_abc = 10;

enum class capi_enumeration : uint8_t {
  capi_enumeration_x = 0,
  capi_enumeration_y = 1,
};

struct capi_struct_abc;

struct capi_union_ghi;

struct capi_struct_def {
  int32_t x;
  float y;
};

union capi_union_jkl {
  int32_t x;
  float y;
};

using capi_type_alias = capi_struct_abc;

static const intptr_t capi_constant_expression = (intptr_t)(capi_type_alias*)10;

extern "C" {

extern const int32_t StaticAbc;

void root(const capi_struct_abc *a,
          capi_struct_def b,
          capi_union_ghi c,
          capi_union_jkl d,
          capi_enumeration e,
          capi_type_alias f);

} // extern "C"

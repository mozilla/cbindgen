#if defined(CBINDGEN_STYLE_TYPE)
/* ANONYMOUS STRUCTS DO NOT SUPPORT FORWARD DECLARATIONS!
#endif


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const TypeInfo *const *fields;
  uintptr_t num_fields;
} StructInfo;

typedef enum {
  Primitive,
  Struct,
} TypeData_Tag;

typedef struct {
  StructInfo _0;
} Struct_Body;

typedef struct {
  TypeData_Tag tag;
  union {
    Struct_Body struct_;
  };
} TypeData;

typedef struct {
  TypeData data;
} TypeInfo;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(TypeInfo x);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#if defined(CBINDGEN_STYLE_TYPE)
*/
#endif

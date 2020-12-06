#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
/* ANONYMOUS STRUCTS DO NOT SUPPORT FORWARD DECLARATIONS!
#endif
#if 0
' '''
#endif


#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct StructInfo {
  const TypeInfo *const *fields;
  uintptr_t num_fields;
};

struct TypeData {
  enum class Tag {
    Primitive,
    Struct,
  };

  Tag tag;
  union {
    struct {
      union {
        StructInfo struct_;
      };
    };
  };
};

struct TypeInfo {
  TypeData data;
};

extern "C" {

void root(TypeInfo x);

} // extern "C"

#if 0
''' '
#endif
#if defined(CBINDGEN_STYLE_TYPE)
*/
#endif
#if 0
' '''
#endif

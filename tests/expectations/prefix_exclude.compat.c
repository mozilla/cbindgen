#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uintptr_t id;
} Struct1;

typedef struct {
  uintptr_t id;
} PREFIX_Struct2;

typedef int32_t PREFIX_Type1[3];

typedef int32_t Type2[15];

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void caller(Struct1 s1, PREFIX_Struct2 s2, PREFIX_Type1 t1, Type2 t2);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

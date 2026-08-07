#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Struct1 {
  uintptr_t id;
} Struct1;

typedef struct PREFIX_Struct2 {
  uintptr_t id;
} PREFIX_Struct2;

typedef int32_t PREFIX_Type1[3];

typedef int32_t Type2[15];

void caller(struct Struct1 s1, struct PREFIX_Struct2 s2, PREFIX_Type1 t1, Type2 t2);

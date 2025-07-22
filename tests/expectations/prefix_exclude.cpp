#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Struct1 {
  uintptr_t id;
};

struct PREFIX_Struct2 {
  uintptr_t id;
};

using PREFIX_Type1 = int32_t[3];

using Type2 = int32_t[15];

extern "C" {

void caller(Struct1 s1, PREFIX_Struct2 s2, PREFIX_Type1 t1, Type2 t2);

}  // extern "C"

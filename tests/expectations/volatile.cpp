#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T>
using V = volatile T;

using Vint = V<int>;

using Vvint = V<Vint>;

using Vpint = V<int*>;

using Vpcint = V<const int*>;

using Vnzu32 = V<uint32_t>;

using Vnnint = V<int*>;

using Vcint = V<int>;

using Vfn = V<void(*)()>;

struct S {
  volatile int vfield;
  V<int> vint;
  V<Vint> vvint;
  V<int*> vpint;
  V<const int*> vpcint;
  V<uint32_t> vnzu32;
  V<int*> vnnint;
  V<int> vcint;
  V<void(*)()> vfn;
  V<int> a1vint[1];
};

union U {
  volatile int vfield;
  V<int> vint;
  V<Vint> vvint;
  V<int*> vpint;
  V<const int*> vpcint;
  V<uint32_t> vnzu32;
  V<int*> vnnint;
  V<void(*)()> vfn;
  V<int> a1vint[1];
};

extern "C" {

extern V<int> g_vint;

extern V<Vint> g_vvint;

extern V<int*> g_vpint;

extern V<const int*> g_vpcint;

extern V<uint32_t> g_vnzu32;

extern V<int*> g_vnnint;

extern V<int> g_vcint;

extern V<void(*)()> g_vfn;

extern V<int> g_a1vint[1];

void _export(Vint, Vvint, Vpint, Vpcint, Vnzu32, Vnnint, Vcint, Vfn, S, U);

}  // extern "C"

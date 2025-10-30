#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Dummy0 {
  uintptr_t dummy;
};

using Dummy0_DummyTrait_DummyOut = Dummy0;

using Dummy0_DummyTrait_DummyIn1 = Dummy0;

using Dummy1_DummyTrait_DummyOut = int32_t;

struct Dummy1 {
  uintptr_t dummy;
};

using Dummy1_DummyTrait_DummyIn1 = uintptr_t;

extern "C" {

Dummy0_DummyTrait_DummyOut dummy_Dummy0(Dummy0 self, Dummy0_DummyTrait_DummyIn1 _in1);

Dummy1_DummyTrait_DummyOut dummy_Dummy1(Dummy1 self, Dummy1_DummyTrait_DummyIn1 _in1);

}  // extern "C"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uintptr_t dummy;
} Dummy0;

typedef Dummy0 Dummy0_DummyTrait_DummyOut;

typedef Dummy0 Dummy0_DummyTrait_DummyIn1;

typedef int32_t Dummy1_DummyTrait_DummyOut;

typedef struct {
  uintptr_t dummy;
} Dummy1;

typedef uintptr_t Dummy1_DummyTrait_DummyIn1;

Dummy0_DummyTrait_DummyOut dummy_Dummy0(Dummy0 self, Dummy0_DummyTrait_DummyIn1 _in1);

Dummy1_DummyTrait_DummyOut dummy_Dummy1(Dummy1 self, Dummy1_DummyTrait_DummyIn1 _in1);

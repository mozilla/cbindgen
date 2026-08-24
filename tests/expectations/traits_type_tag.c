#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Dummy0 {
  uintptr_t dummy;
};

typedef struct Dummy0 Dummy0_DummyTrait_DummyOut;

typedef struct Dummy0 Dummy0_DummyTrait_DummyIn1;

typedef int32_t Dummy1_DummyTrait_DummyOut;

struct Dummy1 {
  uintptr_t dummy;
};

typedef uintptr_t Dummy1_DummyTrait_DummyIn1;

Dummy0_DummyTrait_DummyOut dummy_Dummy0(struct Dummy0 self, Dummy0_DummyTrait_DummyIn1 _in1);

Dummy1_DummyTrait_DummyOut dummy_Dummy1(struct Dummy1 self, Dummy1_DummyTrait_DummyIn1 _in1);

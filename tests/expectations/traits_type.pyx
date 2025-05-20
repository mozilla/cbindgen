from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct Dummy0:
    uintptr_t dummy;

  ctypedef Dummy0 Dummy0_DummyTrait_DummyOut;

  ctypedef Dummy0 Dummy0_DummyTrait_DummyIn1;

  ctypedef int32_t Dummy1_DummyTrait_DummyOut;

  ctypedef struct Dummy1:
    uintptr_t dummy;

  ctypedef uintptr_t Dummy1_DummyTrait_DummyIn1;

  Dummy0_DummyTrait_DummyOut dummy_Dummy0(Dummy0 self, Dummy0_DummyTrait_DummyIn1 _in1);

  Dummy1_DummyTrait_DummyOut dummy_Dummy1(Dummy1 self, Dummy1_DummyTrait_DummyIn1 _in1);

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct DummyStruct:
    int32_t dummy_field;

  DummyStruct new_dummy();

  DummyStruct new_dummy_param(int32_t dummy_field);

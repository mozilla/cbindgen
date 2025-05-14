from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Struct1:
    uintptr_t id;

  cdef struct PREFIX_Struct2:
    uintptr_t id;

  ctypedef int32_t PREFIX_Type1[3];

  ctypedef int32_t Type2[15];

  void caller(Struct1 s1, PREFIX_Struct2 s2, PREFIX_Type1 t1, Type2 t2);

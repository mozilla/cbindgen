from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    A_A1,
    A_A2,
    A_A3,
    # Must be last for serialization purposes
    A_Terminator,
  ctypedef uint8_t A;

  cdef enum:
    B_B1,
    B_B2,
    B_B3,
    # Must be last for serialization purposes
    B_Last,
  ctypedef uint8_t B;

  void root(A a, B b);

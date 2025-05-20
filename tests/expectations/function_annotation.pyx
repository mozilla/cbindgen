from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list
#define TEST_MACRO
#ifndef _In_
#define _In_
#endif


cdef extern from *:

  void root(const uint64_t *input, uint64_t input_size);

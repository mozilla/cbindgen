from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const char C_STRING[] # = "winner"

  const uint8_t BYTES[3] # = [ 97, 98, 99, ]

  const char *ARRAY[2] # = [ "first", "second", ]

  void root();

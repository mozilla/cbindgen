from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint8_t BYTES[3] # = [ 97, 98, 99, ]

  const uint8_t SLICE[19] # = [ 110, 111, 116, 32, 110, 117, 108, 108, 0, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, ]

  const uint8_t BINARY[4] # = [ 0, 255, 16, 127, ]

  const uint8_t EMPTY[0] # = [ ]

  # A documented byte-string constant.
  const uint8_t DOCUMENTED[3] # = [ 100, 111, 99, ]

  void root();

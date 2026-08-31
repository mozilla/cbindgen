from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Reachable from an exported function, so it is emitted even though its crate
  # does not contribute top-level items.
  ctypedef struct UsedDepStruct:
    uint32_t x;

  # Not used by any exported item, but declared by the binding crate.
  ctypedef struct UnusedLocalStruct:
    uint32_t z;

  uint32_t get_x(const UsedDepStruct *used);

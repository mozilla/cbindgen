from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # Matches glob patterns such as `src/*/mod.rs`.
  #
  # A C comment like /* this */ in the docs must not end the generated comment,
  # and neither must `**/*.rs`.
  void root();

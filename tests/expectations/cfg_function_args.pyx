from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  void cfg_function_args(uint32_t first, uint32_t enabled, uint32_t disabled, uint32_t last);

  void cfg_first_argument(uint32_t enabled, uint32_t last);

  void cfg_all_arguments(uint32_t enabled, uint32_t disabled);

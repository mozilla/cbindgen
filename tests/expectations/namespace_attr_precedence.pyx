from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # A function without namespace annotation - should use global namespace
  void uses_global_namespace();

  # A function with per-item namespace - should override global namespace
  void uses_item_namespace(const char *a);

  # Another function without namespace annotation - should use global namespace
  void also_uses_global_namespace();

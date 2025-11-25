from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # A function without namespace annotation - uses global namespace
  void global_function();

  # A function with a single namespace
  void ffi_function();

  # A function with nested namespaces using :: separator
  void nested_function(const char *a);

  # Another function with the same namespace to test grouping
  void another_nested_function();

  # A function with a different nested namespace
  void other_namespace_function();

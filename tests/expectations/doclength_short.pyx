from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # The root of all evil.
  #
  # But at least it contains some more documentation as someone would expect
  # from a simple test case like this. Though, this shouldn't appear in the
  # output.
  void root();

  # A little above the root, and a lot more visible, with a run-on sentence
  # to test going over the first line.
  #
  # Still not here, though.
  void trunk();

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct NotReprC_ManuallyDrop_Point:
    pass

  ctypedef NotReprC_ManuallyDrop_Point Foo;

  ctypedef struct Point:
    int32_t x;
    int32_t y;

  ctypedef struct MyStruct:
    Point point;

  void root(const Foo *a, const MyStruct *with_manual_drop);

  void take(Point with_manual_drop);

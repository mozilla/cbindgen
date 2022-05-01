from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef enum Bar:
    BarSome,
    BarThing,

  ctypedef struct FooU8:
    uint8_t a;

  ctypedef FooU8 Boo;

  ctypedef enum Dog_Tag:
    DogWoof,

  ctypedef struct Dog:
    Dog_Tag tag;
    FooU8 woof;

  void root(Boo x, Bar y, Dog z);

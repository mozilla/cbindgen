from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct MyStruct_1Z:
    uint32_t int_field;

  ctypedef struct S_u32__1Z:
    uint32_t x;

  ctypedef enum Either_1Z__u8_Tag:
    Left_1Z__u8,
    Right_1Z__u8,

  ctypedef struct Either_1Z__u8:
    Either_1Z__u8_Tag tag;

    uint8_t right;

  MyStruct_1Z my_test();

  S_u32__1Z f();

  Either_1Z__u8 returns_either();

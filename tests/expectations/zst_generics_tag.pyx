from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct StructWithGenArray_1Z:
    uint32_t other_field;

  cdef union U_1Z:
    uint32_t b;

  cdef enum E_1Z_Tag:
    A_1Z,
    B_1Z,
    C_1Z,

  cdef struct A_Body_1Z:
    uint32_t y;

  cdef struct E_1Z:
    E_1Z_Tag tag;
    A_Body_1Z a;


  cdef struct CG_1Z__3:
    uint32_t b;

  cdef struct Two_1Z__1Z:
    uint32_t k;

  cdef struct Two_1Z__u8:
    uint8_t r;
    uint32_t k;

  cdef struct Two_u8__1Z:
    uint8_t t;
    uint32_t k;

  cdef struct Wrap1_1Z:
    uint32_t b;

  cdef struct Wrap2_1Z:
    Wrap1_1Z w;
    uint16_t c;

  cdef struct Wrap1_Wrap1_1Z:
    Wrap1_1Z a;
    uint32_t b;

  cdef struct D2_u8:
    uint8_t t;
    uint32_t k;

  cdef struct D2_u8__1Z:
    uint8_t t;
    uint32_t k;

  void use_struct(StructWithGenArray_1Z arg);

  void use_union(U_1Z x);

  void use_enum(E_1Z x);

  void use_cg(CG_1Z__3 x);

  void use_two(Two_1Z__1Z x, Two_1Z__u8 y, Two_u8__1Z z);

  void use_nested(Wrap2_1Z x, Wrap1_Wrap1_1Z y);

  void use_d2(D2_u8 x, D2_u8__1Z y);

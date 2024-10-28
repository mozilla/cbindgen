from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef int V_c_int;

  ctypedef V_c_int Vint;

  ctypedef Vint V_Vint;

  ctypedef V_Vint Vvint;

  ctypedef int *V_____c_int;

  ctypedef V_____c_int Vpint;

  ctypedef const int *V______c_int;

  ctypedef V______c_int Vpcint;

  ctypedef uint32_t V_u32;

  ctypedef V_u32 Vnzu32;

  ctypedef V_____c_int Vnnint;

  ctypedef V_c_int Vcint;

  ctypedef void (*V_______c_void)();

  ctypedef V_______c_void Vfn;

  ctypedef struct S:
    int vfield;
    V_c_int vint;
    V_Vint vvint;
    V_____c_int vpint;
    V______c_int vpcint;
    V_u32 vnzu32;
    V_____c_int vnnint;
    V_c_int vcint;
    V_______c_void vfn;
    V_c_int a1vint[1];

  ctypedef union U:
    int vfield;
    V_c_int vint;
    V_Vint vvint;
    V_____c_int vpint;
    V______c_int vpcint;
    V_u32 vnzu32;
    V_____c_int vnnint;
    V_______c_void vfn;
    V_c_int a1vint[1];

  extern V_c_int g_vint;

  extern V_Vint g_vvint;

  extern V_____c_int g_vpint;

  extern V______c_int g_vpcint;

  extern V_u32 g_vnzu32;

  extern V_____c_int g_vnnint;

  extern V_c_int g_vcint;

  extern V_______c_void g_vfn;

  extern V_c_int g_a1vint[1];

  void _export(Vint, Vvint, Vpint, Vpcint, Vnzu32, Vnnint, Vcint, Vfn, S, U);

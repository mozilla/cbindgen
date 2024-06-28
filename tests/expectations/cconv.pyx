from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  void test_none();

  void test_c();

  void test_cdecl();

  void test_stdcall();

  void test_win64();

  void test_sysv64();

  void test_rust();

  void test_aapcs();

  void test_fastcall();

  void test_thiscall();

  void test_efiapi();

  void test_c();

  void test_cdecl();

  void test_stdcall();

  void test_win64();

  void test_sysv64();

  void test_rust();

  void test_aapcs();

  void test_fastcall();

  void test_thiscall();

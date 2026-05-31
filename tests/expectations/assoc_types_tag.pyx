from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const int64_t CONST_TEST_1 # = 50

  const uint64_t CONST_TEST_2 # = 100

  cdef enum EnumTest_Tag:
    enum_var1,
    enum_var2,
    enum_var3,

  cdef struct enum_var3_Body:
    bool a;
    uint64_t b;

  cdef struct EnumTest:
    EnumTest_Tag tag;
    uint8_t enum_var2[5];
    enum_var3_Body enum_var3;

  cdef struct AnotherStruct:
    uint8_t a;
    int64_t b;
    bool c[36];

  cdef enum UnionTest_Tag:
    union_var1,
    union_var2,
    union_var3,

  cdef struct union_var3_Body:
    bool a;
    uint64_t b;

  cdef struct UnionTest:
    UnionTest_Tag tag;
    uint8_t union_var2[5];
    union_var3_Body union_var3;

  ctypedef int64_t typedef_test;

  extern const uint64_t STATIC_TEST_1[5];

  extern const bool STATIC_TEST_2;

  void test_enum(EnumTest enum_);

  void test_struct_gen(AnotherStruct struct_);

  void test_union(UnionTest union_);

  void test_typedefs(typedef_test typedef_);

  int64_t test_fn(const int64_t *struct_);

  void test_func_ptr(bool (*fn_ptr)(uint64_t, bool));

  void test_raw_ptr(const bool *a, uint64_t *b);

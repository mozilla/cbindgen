from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum TransparentEnumWithNoData:
    NoData,

  cdef struct DummyStruct:
    pass

  cdef struct EnumWithAssociatedConstantInImpl:
    pass

  cdef struct StructWithAssociatedConstantInImpl:
    pass

  ctypedef DummyStruct TransparentComplexWrappingStructTuple;

  ctypedef uint32_t TransparentPrimitiveWrappingStructTuple;

  ctypedef DummyStruct TransparentComplexWrappingStruct;

  ctypedef uint32_t TransparentPrimitiveWrappingStruct;

  ctypedef DummyStruct TransparentComplexWrapperStruct_i32;

  ctypedef uint32_t TransparentPrimitiveWrapperStruct_i32;

  ctypedef uint32_t TransparentPrimitiveStructWithAssociatedConstants;
  const TransparentPrimitiveStructWithAssociatedConstants TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ZERO # = 0
  const TransparentPrimitiveStructWithAssociatedConstants TransparentPrimitiveStructWithAssociatedConstants_STRUCT_ONE # = 1

  cdef struct TransparentEmptyStruct:
    pass

  ctypedef const uint32_t *TransparentPointerWrappingStruct;

  ctypedef int32_t TransparentIntStruct;

  ctypedef DummyStruct TransparentComplexStruct;

  ctypedef TransparentPrimitiveWrappingStruct TransparentTransparentStruct;

  ctypedef uint32_t *TransparentNonNullStruct;

  ctypedef uint32_t *TransparentOptionNonNullStruct;

  ctypedef DummyStruct TransparentComplexWrappingEnumTuple;

  ctypedef uint32_t TransparentPrimitiveWrappingEnumTuple;

  ctypedef DummyStruct TransparentComplexWrappingEnum;

  ctypedef uint32_t TransparentPrimitiveWrappingEnum;

  ctypedef DummyStruct TransparentComplexWrapperEnum_i32;

  ctypedef uint32_t TransparentPrimitiveWrapperEnum_i32;

  cdef enum TransparentEnumWithEmptyData_Tag:
    EmptyData,

  cdef struct EmptyData_Body:
    pass

  cdef struct TransparentEnumWithEmptyData:
    TransparentEnumWithEmptyData_Tag tag;
    EmptyData_Body empty_data;

  ctypedef uint32_t TransparentPrimitiveEnumWithAssociatedConstants;

  ctypedef const uint32_t *TransparentPointerWrappingEnum;

  ctypedef int32_t TransparentIntEnum;

  ctypedef DummyStruct TransparentComplexEnum;

  ctypedef TransparentPrimitiveWrappingEnum TransparentTransparentEnum;

  ctypedef uint32_t *TransparentNonNullEnum;

  ctypedef uint32_t *TransparentOptionNonNullEnum;

  const TransparentPrimitiveWrappingStruct StructWithAssociatedConstantInImpl_STRUCT_TEN # = 10





  const TransparentPrimitiveWrappingStruct EnumWithAssociatedConstantInImpl_ENUM_TEN # = 10

  void struct_root(TransparentComplexWrappingStructTuple a,
                   TransparentPrimitiveWrappingStructTuple b,
                   TransparentComplexWrappingStruct c,
                   TransparentPrimitiveWrappingStruct d,
                   TransparentComplexWrapperStruct_i32 e,
                   TransparentPrimitiveWrapperStruct_i32 f,
                   TransparentPrimitiveStructWithAssociatedConstants g,
                   TransparentEmptyStruct h,
                   TransparentPointerWrappingStruct i,
                   StructWithAssociatedConstantInImpl j);

  void erased_root(uint32_t *a,
                   uint32_t *b,
                   TransparentPrimitiveWrappingStruct c,
                   uint32_t *d,
                   TransparentIntStruct e,
                   int32_t f,
                   DummyStruct g,
                   uint32_t *h,
                   int32_t i,
                   TransparentIntStruct j,
                   TransparentComplexStruct k,
                   TransparentTransparentStruct l,
                   TransparentNonNullStruct m,
                   TransparentOptionNonNullStruct n);

  void enum_root(TransparentComplexWrappingEnumTuple a,
                 TransparentPrimitiveWrappingEnumTuple b,
                 TransparentComplexWrappingEnum c,
                 TransparentPrimitiveWrappingEnum d,
                 TransparentComplexWrapperEnum_i32 e,
                 TransparentPrimitiveWrapperEnum_i32 f,
                 TransparentEnumWithNoData g,
                 TransparentEnumWithEmptyData h,
                 TransparentPrimitiveEnumWithAssociatedConstants i,
                 TransparentPointerWrappingEnum j,
                 EnumWithAssociatedConstantInImpl k);

  void erased_enum_root(uint32_t *a,
                        uint32_t *b,
                        TransparentPrimitiveWrappingEnum c,
                        uint32_t *d,
                        TransparentIntEnum e,
                        int32_t f,
                        DummyStruct g,
                        uint32_t *h,
                        int32_t i,
                        TransparentIntEnum j,
                        TransparentComplexEnum k,
                        TransparentTransparentEnum l,
                        TransparentNonNullEnum m,
                        TransparentOptionNonNullEnum n);

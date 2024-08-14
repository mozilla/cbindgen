struct DummyStruct;

// Transparent struct tuple wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStructTuple(DummyStruct);

// Transparent struct tuple wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStructTuple(u32);

// Transparent struct wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStruct { only_field: DummyStruct }

// Transparent struct wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStruct { only_field: u32 }

// Transparent struct wrapping a pointer
#[repr(transparent)]
struct TransparentPointerWrappingStruct { only_field: *const u32 }

// Transparent struct wrapper with a marker wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrapperStruct<T> {
    only_non_zero_sized_field: DummyStruct,
    marker: PhantomData<T>,
}

// Transparent struct wrapper with a marker wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrapperStruct<T> {
    only_non_zero_sized_field: u32,
    marker: PhantomData<T>,
}

// Associated constant declared before struct declaration.
impl TransparentPrimitiveStructWithAssociatedConstants {
    pub const STRUCT_ZERO: TransparentPrimitiveStructWithAssociatedConstants =
        TransparentPrimitiveStructWithAssociatedConstants { bits: 0 };
}

// Transparent struct wrapping a primitive with associated constants.
#[repr(transparent)]
struct TransparentPrimitiveStructWithAssociatedConstants { bits: u32 }

// Transparent zero-sized structs are legal rust, but there's no way to emit a typedef for one, so
// cbindgen should treat it as repr(C) instead and emit an empty struct definition.
#[repr(transparent)]
struct TransparentEmptyStruct;

// Associated constant declared after struct declaration.
impl TransparentPrimitiveStructWithAssociatedConstants {
    pub const STRUCT_ONE: TransparentPrimitiveStructWithAssociatedConstants =
        TransparentPrimitiveStructWithAssociatedConstants { bits: 1 };
}

struct StructWithAssociatedConstantInImpl { }

impl StructWithAssociatedConstantInImpl {
    pub const STRUCT_TEN: TransparentPrimitiveWrappingStruct =
        TransparentPrimitiveWrappingStruct { only_field: 10 };
}

#[no_mangle]
pub extern "C" fn struct_root(
    a: TransparentComplexWrappingStructTuple,
    b: TransparentPrimitiveWrappingStructTuple,
    c: TransparentComplexWrappingStruct,
    d: TransparentPrimitiveWrappingStruct,
    e: TransparentComplexWrapperStruct<i32>,
    f: TransparentPrimitiveWrapperStruct<i32>,
    g: TransparentPrimitiveStructWithAssociatedConstants,
    h: TransparentEmptyStruct,
    i: TransparentPointerWrappingStruct,
    j: StructWithAssociatedConstantInImpl,
) { }

#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentNonNullPointerWrappingStruct { only_field: NonNull<u32> }

#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentOptionalNonNullPointerWrappingStruct { only_field: Option<NonNull<u32>> }

#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentWrappingAnotherTransparentStruct { only_field: TransparentPrimitiveWrappingStruct }

/// cbindgen:transparent-typedef
#[repr(transparent)]
struct ErasedTransparentWrappingTransparentNonNullPointerStruct { only_field: ErasedTransparentNonNullPointerWrappingStruct }

// Transparent struct wrapping another type
#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentStructWrappingAnotherType<T> { only_field: T }

type TransparentIntStruct = ErasedTransparentStructWrappingAnotherType<i32>;
type TransparentComplexStruct = ErasedTransparentStructWrappingAnotherType<DummyStruct>;
type TransparentTransparentStruct = ErasedTransparentStructWrappingAnotherType<TransparentPrimitiveWrappingStruct>;
type TransparentNonNullStruct = ErasedTransparentStructWrappingAnotherType<NonNull<u32>>;
type TransparentOptionNonNullStruct = ErasedTransparentStructWrappingAnotherType<Option<NonNull<u32>>>;

/// cbindgen:transparent-typedef
type ErasedTransparentIntStruct = ErasedTransparentStructWrappingAnotherType<i32>;
/// cbindgen:transparent-typedef
type ErasedTransparentComplexStruct = ErasedTransparentStructWrappingAnotherType<DummyStruct>;
/// cbindgen:transparent-typedef
type ErasedTransparentOptionNonNullStruct = ErasedTransparentStructWrappingAnotherType<Option<NonNull<u32>>>;

#[no_mangle]
pub extern "C" fn erased_root(
    a: ErasedTransparentNonNullPointerWrappingStruct,
    b: ErasedTransparentOptionalNonNullPointerWrappingStruct,
    c: ErasedTransparentWrappingAnotherTransparentStruct,
    d: ErasedTransparentWrappingTransparentNonNullPointerStruct,
    e: ErasedTransparentStructWrappingAnotherType<TransparentIntStruct>,
    f: ErasedTransparentStructWrappingAnotherType<ErasedTransparentIntStruct>,
    g: ErasedTransparentStructWrappingAnotherType<ErasedTransparentComplexStruct>,
    h: ErasedTransparentStructWrappingAnotherType<ErasedTransparentOptionNonNullStruct>,
    i: ErasedTransparentIntStruct,
    j: TransparentIntStruct,
    k: TransparentComplexStruct,
    l: TransparentTransparentStruct,
    m: TransparentNonNullStruct,
    n: TransparentOptionNonNullStruct,
) { }

// Transparent enum tuple wrapping a struct.
#[repr(transparent)]
enum TransparentComplexWrappingEnumTuple {
    A(DummyStruct),
}

// Transparent enum tuple wrapping a primitive.
#[repr(transparent)]
enum TransparentPrimitiveWrappingEnumTuple {
    A(u32),
}

// Transparent enum wrapping a struct.
#[repr(transparent)]
enum TransparentComplexWrappingEnum {
    A { only_field: DummyStruct },
}

// Transparent enum wrapping a primitive.
#[repr(transparent)]
enum TransparentPrimitiveWrappingEnum {
    A { only_field: u32 },
}

// Transparent enum wrapping a pointer
#[repr(transparent)]
enum TransparentPointerWrappingEnum {
    A { only_field: *const u32 },
}

// Transparent enum wrapper with a marker wrapping a struct.
#[repr(transparent)]
enum TransparentComplexWrapperEnum<T> {
    A { only_non_zero_sized_field: DummyStruct },
    B { marker: PhantomData<T> },
}

// Transparent enum wrapper with a marker wrapping a primitive.
#[repr(transparent)]
enum TransparentPrimitiveWrapperEnum<T> {
    C { only_non_zero_sized_field: u32 },
    D { marker: PhantomData<T> },
}

// Transparent enums with missing or zero-sized data are legal rust, but there's no way to emit a
// transparent typedef for them, so cbindgen should treat them as repr(C) instead.
#[repr(transparent)]
enum TransparentEnumWithNoData {
    NoData,
}
#[repr(transparent)]
enum TransparentEnumWithEmptyData {
    EmptyData(),
}

// Associated constant declared before enum declaration.
impl TransparentPrimitiveEnumWithAssociatedConstants {
    // TODO: Transparent enum constants are not supported yet.
    pub const ENUM_ZERO: TransparentPrimitiveEnumWithAssociatedConstants =
        TransparentPrimitiveEnumWithAssociatedConstants::A { bits: 0 };
}

// Transparent enum wrapping a primitive with associated constants.
#[repr(transparent)]
enum TransparentPrimitiveEnumWithAssociatedConstants {
    A { bits: u32 },
}

// Associated constant declared after enum declaration.
impl TransparentPrimitiveEnumWithAssociatedConstants {
    // TODO: Transparent enum constants are not supported yet.
    pub const ENUM_ONE: TransparentPrimitiveEnumWithAssociatedConstants =
        TransparentPrimitiveEnumWithAssociatedConstants::A { bits: 1 };
}

enum EnumWithAssociatedConstantInImpl { A }

impl EnumWithAssociatedConstantInImpl {
    pub const ENUM_TEN: TransparentPrimitiveWrappingStruct =
        TransparentPrimitiveWrappingStruct { only_field: 10 };
}

#[no_mangle]
pub extern "C" fn enum_root(
    a: TransparentComplexWrappingEnumTuple,
    b: TransparentPrimitiveWrappingEnumTuple,
    c: TransparentComplexWrappingEnum,
    d: TransparentPrimitiveWrappingEnum,
    e: TransparentComplexWrapperEnum<i32>,
    f: TransparentPrimitiveWrapperEnum<i32>,
    g: TransparentEnumWithNoData,
    h: TransparentEnumWithEmptyData,
    i: TransparentPrimitiveEnumWithAssociatedConstants,
    j: TransparentPointerWrappingEnum,
    k: EnumWithAssociatedConstantInImpl,
) { }

#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentNonNullPointerWrappingEnum { A(NonNull<u32>) }

#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentOptionalNonNullPointerWrappingEnum { A(Option<NonNull<u32>>) }

#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentWrappingAnotherTransparentEnum { A(TransparentPrimitiveWrappingEnum) }

/// cbindgen:transparent-typedef
#[repr(transparent)]
enum ErasedTransparentWrappingTransparentNonNullPointerEnum { A(ErasedTransparentNonNullPointerWrappingEnum) }

// Transparent enumure wrapping another type
#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentEnumWrappingAnotherType<T> { A(T) }

type TransparentIntEnum = ErasedTransparentEnumWrappingAnotherType<i32>;
type TransparentComplexEnum = ErasedTransparentEnumWrappingAnotherType<DummyStruct>;
type TransparentTransparentEnum = ErasedTransparentEnumWrappingAnotherType<TransparentPrimitiveWrappingEnum>;
type TransparentNonNullEnum = ErasedTransparentEnumWrappingAnotherType<NonNull<u32>>;
type TransparentOptionNonNullEnum = ErasedTransparentEnumWrappingAnotherType<Option<NonNull<u32>>>;

/// cbindgen:transparent-typedef
type ErasedTransparentIntEnum = ErasedTransparentEnumWrappingAnotherType<i32>;
/// cbindgen:transparent-typedef
type ErasedTransparentComplexEnum = ErasedTransparentEnumWrappingAnotherType<DummyStruct>;
/// cbindgen:transparent-typedef
type ErasedTransparentOptionNonNullEnum = ErasedTransparentEnumWrappingAnotherType<Option<NonNull<u32>>>;

#[no_mangle]
pub extern "C" fn erased_enum_root(
    a: ErasedTransparentNonNullPointerWrappingEnum,
    b: ErasedTransparentOptionalNonNullPointerWrappingEnum,
    c: ErasedTransparentWrappingAnotherTransparentEnum,
    d: ErasedTransparentWrappingTransparentNonNullPointerEnum,
    e: ErasedTransparentEnumWrappingAnotherType<TransparentIntEnum>,
    f: ErasedTransparentEnumWrappingAnotherType<ErasedTransparentIntEnum>,
    g: ErasedTransparentEnumWrappingAnotherType<ErasedTransparentComplexEnum>,
    h: ErasedTransparentEnumWrappingAnotherType<ErasedTransparentOptionNonNullEnum>,
    i: ErasedTransparentIntEnum,
    j: TransparentIntEnum,
    k: TransparentComplexEnum,
    l: TransparentTransparentEnum,
    m: TransparentNonNullEnum,
    n: TransparentOptionNonNullEnum,
) { }

#[repr(transparent)]
struct TransparentStruct { field: u8 }

impl TransparentStruct {
    pub const ASSOC_STRUCT_FOO: i64 = 1;
    pub const ASSOC_STRUCT_BAR: TransparentStruct = TransparentStruct { field: 2 };

    // TODO: Only C++ supports template constants so far.
    pub const ASSOC_STRUCT_BAZ: Wrapper<TransparentStruct> = Wrapper { field: TransparentStruct { field: 3 } };
}

#[repr(transparent)]
struct TransparentTupleStruct(u8);

/// cbindgen:transparent-typedef
#[repr(transparent)]
struct Wrapper<T> { field: T }

pub const STRUCT_FOO: TransparentStruct = TransparentStruct { field: 4 };
pub const STRUCT_BAR: TransparentTupleStruct = TransparentTupleStruct(5);

// TODO: Only C++ supports template constants so far.
pub const STRUCT_BAZ: Wrapper<TransparentStruct> = Wrapper { field: TransparentStruct { field: 6 } };

#[repr(transparent)]
struct TransparentStructWithErasedField<T> {
    field: Wrapper<T>,
}

// TODO: Only C++ supports template constants so far.
pub const COMPLEX: TransparentStructWithErasedField<TransparentStruct> = TransparentStructWithErasedField {
    field: Wrapper { field: TransparentStruct { field: 7 } }
};

#[repr(transparent)]
enum TransparentEnum {
    A { field: u8 },
}

impl TransparentEnum {
    pub const ASSOC_ENUM_FOO: i64 = 8;

    // TODO: Transparent enum constants are not supported yet.
    pub const ASSOC_ENUM_BAR: TransparentEnum = TransparentEnum::A { field: 9 };
    pub const ASSOC_ENUM_BAZ: TransparentWrapperEnum<TransparentEnum> = TransparentWrapperEnum::A {
        field: TransparentEnum::A { field: 10 }
    };
}

#[repr(transparent)]
enum TransparentTupleEnum {
    A(u8),
}

#[repr(transparent)]
enum TransparentWrapperEnum<T> {
    A { field: T },
}

// TODO: Transparent enum constants are not supported yet.
pub const ENUM_FOO: TransparentEnum = TransparentEnum::A { field: 11 };
pub const ENUM_BAR: TransparentTupleEnum = TransparentTupleEnum::A(12);
pub const ENUM_BAZ: TransparentWrapperEnum<TransparentEnum> = TransparentWrapperEnum::A {
    field: TransparentEnum::A { field: 13 }
};

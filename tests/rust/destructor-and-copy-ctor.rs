use std::ptr::NonNull;

/// This will have a destructor manually implemented via variant_body, and
/// similarly a Drop impl in Rust.
#[repr(C)]
pub struct OwnedSlice<T> {
    len: usize,
    ptr: NonNull<T>,
}

#[repr(u8)]
pub enum FillRule { A, B }

#[repr(C)]
pub struct Polygon<LengthPercentage> {
    pub fill: FillRule,
    pub coordinates: OwnedSlice<LengthPercentage>,
}

#[repr(C, u8)]
pub enum Foo<T> {
    Bar,
    Polygon1(Polygon<T>),
    Slice1(OwnedSlice<T>),
    Slice2(OwnedSlice<i32>),
    Slice3 {
        fill: FillRule,
        coords: OwnedSlice<T>,
    },
    Slice4 {
        fill: FillRule,
        coords: OwnedSlice<i32>,
    },
}

#[repr(u8)]
pub enum Baz<T> {
    Bar2,
    Polygon21(Polygon<T>),
    Slice21(OwnedSlice<T>),
    Slice22(OwnedSlice<i32>),
    Slice23 {
        fill: FillRule,
        coords: OwnedSlice<T>,
    },
    Slice24 {
        fill: FillRule,
        coords: OwnedSlice<i32>,
    },
}

#[repr(u8)]
pub enum Taz {
    Bar3,
    Taz1(i32),
    Taz3(OwnedSlice<i32>),
}

/// cbindgen:derive-tagged-enum-destructor=false
/// cbindgen:derive-tagged-enum-copy-constructor=false
#[repr(u8)]
pub enum Tazz {
    Bar4,
    Taz2(i32),
}

/// cbindgen:derive-tagged-enum-copy-assignment=false
#[repr(u8)]
pub enum Tazz {
    Bar4,
    Taz2(i32),
}

#[no_mangle]
pub extern "C" fn root(a: &Foo<u32>, b: &Baz<i32>, c: &Taz, d: Tazz) {}

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

/// cbindgen:destructor
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

#[no_mangle]
pub extern "C" fn root(p: &Foo<u32>) {}

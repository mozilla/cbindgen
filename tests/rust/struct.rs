use std::marker::PhantomData;

struct Opaque {
    x: i32,
    y: f32,
}

#[repr(C)]
struct Normal {
    x: i32,
    y: f32,
}

#[repr(C)]
struct NormalWithZST {
    x: i32,
    y: f32,
    z: (),
    w: PhantomData<i32>,
    v: PhantomPinned,
}

/// cbindgen:rename-all=GeckoCase
#[repr(C)]
struct TupleRenamed(i32, f32);

/// cbindgen:field-names=[x, y]
#[repr(C)]
struct TupleNamed(i32, f32);

#[repr(C)]
struct WithFlexibleArrayMember {
    x: i32,
    y: [i16; 0],
    z: [i8; 0],
}

#[no_mangle]
pub extern "C" fn root(
    a: *mut Opaque,
    b: Normal,
    c: NormalWithZST,
    d: TupleRenamed,
    e: TupleNamed,
    f: WithFlexibleArrayMember,
) { }

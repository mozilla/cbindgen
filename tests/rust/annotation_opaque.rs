/// cbindgen:opaque
#[repr(C)]
struct A(i32);

/// cbindgen:opaque
#[repr(C)]
struct B(i32, f32);

/// cbindgen:opaque
#[repr(u32)]
enum C {
    X = 2,
    Y,
}

/// cbindgen:opaque
#[repr(u8)]
enum F {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

/// cbindgen:opaque
#[repr(C, u8)]
enum H {
    Hello(i16),
    There { x: u8, y: i16 },
    Everyone,
}

/// cbindgen:opaque
#[repr(transparent)]
struct P(*mut i32);

/// cbindgen:opaque
#[repr(transparent)]
struct T(u16);

#[no_mangle]
pub extern "C" fn root(x: A, y: B, z: C, f: F, h: H, p: P, t: T) {}

/// cbindgen:opaque
type NumberPtr = Option<NonNull<u16>>;
// (has no effect)

type BytePtr = Option<NonNull<u8>>;

#[no_mangle]
pub extern "C" fn bytes_and_nums(b: BytePtr, n: NumberPtr, bp: *mut BytePtr, np: *mut NumberPtr) {}

// Explicit opaqueness always has priority, except for `type`s which can't be opaque.

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(transparent)]
struct P(*mut i32);

/// cbindgen:transparent-typedef
#[repr(transparent)]
struct T(u16);

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(transparent)]
struct O(u16);

#[no_mangle]
pub extern "C" fn root(p: P, pp: *mut P, t: T, tp: *mut T, o: O, op: *mut O) {}

/// cbindgen:opaque
/// cbindgen:transparent-typedef
type NumberPtr = Option<NonNull<u16>>;
// (has no effect)

/// cbindgen:transparent-typedef
type BytePtr = Option<NonNull<u8>>;

#[no_mangle]
pub extern "C" fn bytes_and_nums(b: BytePtr, n: NumberPtr, bp: *mut BytePtr, np: *mut NumberPtr) {}

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(C)]
struct A(i32);

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(C)]
struct B(i32, f32);

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(u32)]
enum C {
    X = 2,
    Y,
}

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(u8)]
enum F {
    Foo(i16),
    Bar { x: u8, y: i16 },
    Baz,
}

/// cbindgen:transparent-typedef
/// cbindgen:opaque
#[repr(C, u8)]
enum H {
    Hello(i16),
    There { x: u8, y: i16 },
    Everyone,
}

#[no_mangle]
pub extern "C" fn other(x: A, y: B, z: C, f: F, h: H) {}

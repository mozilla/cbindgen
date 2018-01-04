/// cbindgen:derive-lt=true
/// cbindgen:derive-lte=true
/// cbindgen:rename-all=GeckoCase
#[repr(C)]
struct A(i32);

/// cbindgen:field-names=[x, y]
#[repr(C)]
struct B(i32, f32);

/// cbindgen:trailing-values=[Z, W]
#[repr(u32)]
enum C {
    X = 2,
    Y,
}

#[no_mangle]
pub extern "C" fn root(
    x: A,
    y: B,
    z: C
) { }

/// cbindgen:struct-gen-op-lt=true
/// cbindgen:struct-gen-op-lte=true
#[repr(C)]
struct A(i32);

/// cbindgen:field-names=[x, y]
#[repr(C)]
struct B(i32, f32);

/// cbindgen:enum-trailing-values=[Z, W]
#[repr(u32)]
enum C {
    X = 2,
    Y,
}

/// cbindgen:function-prefix=PREFIX
/// cbindgen:function-postfix=PREFIX
#[no_mangle]
extern "C" fn root(x: A, y: B, z: C) {
}

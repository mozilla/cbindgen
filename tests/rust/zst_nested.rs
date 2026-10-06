#[repr(C)]
pub enum MyOption<T> {
    Some(T),
    None,
}

#[repr(C)]
pub struct MyWrapper<T> {
    inner: T,
}

struct Opaque<T> {
    inner: T,
}

#[repr(C)]
pub struct S<T> {
    args_erased: extern "C" fn(arg: T),

    args_erased_with_npo: Option<extern "C" fn(arg: T)>,
    args_erased_with_custom_enum: MyOption<extern "C" fn(arg: T)>,
    args_erased_with_custom_struct: MyWrapper<extern "C" fn(arg: T)>,
    args_erased_with_opaque: *const Opaque<extern "C" fn(arg: T)>,

    args_erased_behind_ptr: *const extern "C" fn(arg: T),
    args_erased_in_array: [extern "C" fn(arg: T); 2],
    args_erased_in_ret: extern "C" fn() -> extern "C" fn(arg: T),
    
    arr_erased: [T; 2],
    arr_erased_behind_ptr: *const [T; 2],
    arr_erased_in_opaque: *const Opaque<[T; 2]>,
    // positions where a ZST is valid are tested in zst_valid.rs
}

#[repr(C)]
pub struct S2<T, U> { 
    f: fn(T, U),
    x: U,
}

#[repr(C)]
pub struct Inner<U> {
    p: *const U,
}

#[repr(C)]
pub struct Outer<T> {
    plain: Inner<T>,
    arr: Inner<[T; 2]>,
}

/// cbindgen:test-expect-warning-cpp=C++ bindings for S (instantiated as S_1Z) may be ill-formed
/// cbindgen:test-expect-warning-cpp=field(s) args_erased, args_erased_with_npo, args_erased_with_custom_enum, args_erased_with_custom_struct, args_erased_with_opaque, args_erased_behind_ptr, args_erased_in_array, args_erased_in_ret, arr_erased, arr_erased_behind_ptr, arr_erased_in_opaque are
/// cbindgen:test-expect-warning-cpp=C++ bindings for S2 (instantiated as S2_1Z___________u8__________4) may be ill-formed
/// cbindgen:test-expect-warning-cpp=field(s) f are
/// cbindgen:test-expect-warning-cpp=C++ bindings for Outer (instantiated as Outer_1Z) may be ill-formed
/// cbindgen:test-expect-warning-cpp=field(s) arr are
#[no_mangle]
pub extern "C" fn use_S(arg1: S<()>, arg2: S2<(), [u8; 4]>, arg3: Outer<()>) {}
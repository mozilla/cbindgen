//! cbindgen:test-expect-no-warnings
// ZST generic arguments that must not produce any warnings.

use std::marker::{PhantomData, PhantomPinned};

#[no_mangle]
pub extern "C" fn option_u32(arg: Option<u32>) {}

#[no_mangle]
pub extern "C" fn option_unit(arg: Option<()>, nested: Option<Option<()>>) {}

#[no_mangle]
pub extern "C" fn result_zsts(arg1: Result<(), u32>, arg2: Result<u32, PhantomData<u32>>, arg3: Result<(), PhantomPinned>) {}


#[repr(C)]
pub struct CG<T, const N: usize> { a: [T; N], b: u32 }

#[no_mangle]
pub extern "C" fn use_cg(x: CG<u8, 3>) {}


struct Opaque<T> {
    inner: T,
}

// Positions where a ZST argument is valid in C++ as well
#[repr(C)]
pub struct ValidPositions<T> {
    args_always_erased: extern "C" fn(arg: ()), // does not depend on T
    zst_behind_ptr: *const T, // *void is valid
    zst_in_opaque: *const Opaque<T>, // *Opaque<void> is valid
    f_returns_zst: extern "C" fn(arg: u32) -> T, // void return is valid
}

#[no_mangle]
pub extern "C" fn use_valid_positions(arg: ValidPositions<()>) {}


// --------------------------
// #527, example by TheBlueMatt (simplified): pointers to ZST can be safely represented as *void
#[repr(C)]
pub struct CResultTempl<O, E> {
	pub result_good: bool,
	pub result: *const O,
	pub err: *const E,
}

// simplified the rest of example by removing unknown types and replacing static with function
#[repr(C)]
pub struct APIError {
    err: u32,
}

#[no_mangle]
pub type CResultNoneAPIError = CResultTempl<(), APIError>;

#[no_mangle]
pub extern "C" fn CResultNoneAPIError_free(_res: CResultNoneAPIError) {}
// --------------------------


// --------------------------
// #659, a simpler case by stevenengler
// _phantom field of Foo is zero-sized regardless of T, so can always be removed
#[repr(C)]
struct Foo<T> {
    x: u32,
    _phantom: PhantomData<T>,
}

#[no_mangle]
pub extern "C" fn bar() -> Foo<()> {
    todo!()
}
// --------------------------

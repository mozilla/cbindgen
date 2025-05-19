//! This test uses the `volatile` annotation to generate C volatile types.

use std::ffi::c_int;
use std::num::NonZeroU32;
use std::ptr::addr_of_mut;
use std::ptr::null_mut;
use std::ptr::read_volatile;
use std::ptr::write_volatile;
use std::ptr::NonNull;
use std::cell::Cell;

// C volatile variable that is managed with a transparent wrapper in rust
/// cbindgen:volatile
#[repr(transparent)]
pub struct V<T>(T);
impl<T> V<T> {
    pub const fn new(x: T) -> Self {
        V(x)
    }
    pub fn get(&self) -> T {
        unsafe { read_volatile(&self.0) }
    }
    pub fn set(&mut self, x: T) {
        unsafe { write_volatile(&mut self.0, x) }
    }
}
impl<T: Clone> Clone for V<T> {
    fn clone(&self) -> Self {
        V(self.0.clone())
    }
}
impl<T: Copy> Copy for V<T> {}

pub type Vint = V<c_int>;

pub type Vvint = V<Vint>;

pub type Vpint = V<*mut c_int>;

pub type Vpcint = V<*const c_int>;

pub type Vnzu32 = V<NonZeroU32>;

pub type Vnnint = V<NonNull<c_int>>;

pub type Vcint = V<Cell<c_int>>;

pub type Vfn = V<Option<extern "C" fn()>>;

// TODO how do you represent array types in a FFI-safe way?

#[repr(C)]
pub struct S {
    // C volatile struct field that is managed manually in rust
    /// cbindgen:volatile
    vfield: c_int,
    pub vint: V<c_int>,
    pub vvint: V<Vint>,
    pub vpint: V<*mut c_int>,
    pub vpcint: V<*const c_int>,
    pub vnzu32: V<NonZeroU32>,
    pub vnnint: V<NonNull<c_int>>,
    pub vcint: V<Cell<c_int>>,
    pub vfn: V<Option<extern "C" fn()>>,
    pub a1vint: [V<c_int>; 1],
}
impl S {
    pub fn vfield(&self) -> c_int {
        unsafe { read_volatile(&self.vfield) }
    }
    pub fn set_vfield(&mut self, x: c_int) {
        unsafe { write_volatile(&mut self.vfield, x) }
    }
}

#[repr(C)]
pub union U {
    // C volatile union field that is managed manually in rust
    /// cbindgen:volatile
    vfield: c_int,
    pub vint: V<c_int>,
    pub vvint: V<Vint>,
    pub vpint: V<*mut c_int>,
    pub vpcint: V<*const c_int>,
    pub vnzu32: V<NonZeroU32>,
    pub vnnint: V<NonNull<c_int>>,
    // TODO unions require Copy or ManuallyDrop. Cell is not Copy and ManuallyDrop fails in Cpp because it is opaque instead of transparent?
    //pub vcint: std::mem::ManuallyDrop<V<Cell<c_int>>>,
    pub vfn: V<Option<extern "C" fn()>>,
    pub a1vint: [V<c_int>; 1],
}
impl U {
    pub fn vfield(&self) -> c_int {
        unsafe { read_volatile(&self.vfield) }
    }
    pub fn set_vfield(&mut self, x: c_int) {
        unsafe { write_volatile(&mut self.vfield, x) }
    }
}

static mut G_INT: c_int = 0;

#[no_mangle]
pub static mut g_vint: V<c_int> = V::new(0);

#[no_mangle]
pub static mut g_vvint: V<Vint> = V::new(Vint::new(0));

#[no_mangle]
pub static mut g_vpint: V<*mut c_int> = V::new(null_mut());

#[no_mangle]
pub static mut g_vpcint: V<*const c_int> = V::new(null_mut());

#[no_mangle]
pub static mut g_vnzu32: V<NonZeroU32> = unsafe { V::new(NonZeroU32::new_unchecked(1)) };

#[no_mangle]
pub static mut g_vnnint: V<NonNull<c_int>> = unsafe { V::new(NonNull::new_unchecked(addr_of_mut!(G_INT))) };

#[no_mangle]
pub static mut g_vcint: V<Cell<c_int>> = V::new(Cell::new(0));

#[no_mangle]
pub static mut g_vfn: V<Option<extern "C" fn()>> = V::new(None);

#[no_mangle]
pub static mut g_a1vint: [V<c_int>; 1] = [V::new(0)];

#[no_mangle]
pub extern "C" fn _export(
    _: Vint,
    _: Vvint,
    _: Vpint,
    _: Vpcint,
    _: Vnzu32,
    _: Vnnint,
    _: Vcint,
    _: Vfn,
    _: S,
    _: U,
) { }

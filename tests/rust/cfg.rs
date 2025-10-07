#[cfg(all(unix, x11))]
#[repr(u32)]
enum FooType {
  A,
  B,
  C,
}


#[repr(C)]
pub struct Flags(u8);
bitflags! {
    impl Flags: u8 {
        /// none
        const NONE = 0;
        #[cfg(windows)]
        const A = 1 << 0;
        #[cfg(unix)]
        const A = 1 << 1;

        #[cfg(windows)]
        const B = Self::A.bits() | (1 << 3);
        #[cfg(unix)]
        const B = Self::A.bits() | (1 << 4);
    }
}

#[cfg(all(unix, x11))]
#[repr(C)]
struct FooHandle {
    ty: FooType,
    flags: Flags,
    x: i32,
    y: f32,
}

#[cfg(any(windows, target_pointer_width="32"))]
#[repr(u32)]
enum BarType {
  A,
  B,
  C,
}

#[repr(u8)]
pub enum C {
    C1,
    C2,
    #[cfg(windows)]
    C3,
    #[cfg(unix)]
    C5 { int: i32 },
}

#[cfg(any(windows, target_pointer_width="32"))]
#[repr(C)]
struct BarHandle {
    ty: BarType,
    x: i32,
    y: f32,
}

// FIXME(#634): Support deriving methods for structs with conditional fields.
/// cbindgen:derive-eq=false
/// cbindgen:derive-neq=false
#[repr(C)]
struct ConditionalField {
    #[cfg(x11)]
    field: i32,
}

impl ConditionalField {
    pub const ZERO: Self = Self {
        #[cfg(x11)]
        field: 0,
    };
    pub const ONE: Self = Self {
        #[cfg(x11)]
        field: 1,
    };
}

#[cfg(all(unix, x11))]
#[no_mangle]
pub extern "C" fn root(a: FooHandle, c: C)
{ }

#[cfg(any(windows, target_pointer_width="32"))]
#[no_mangle]
pub extern "C" fn root(a: BarHandle, c: C)
{ }

#[no_mangle]
pub extern "C" fn cond(a: ConditionalField)
{ }

// src/lib.rs
#[repr(C)]
struct Normal {
    x: i32,
    y: f32,
}

#[cfg(windows)]
extern "C" {
    fn foo() -> i32;

    fn bar(a: Normal);
}

#[cfg(windows)]
#[no_mangle]
pub static mut global_array_with_different_sizes: [i32; 2] = [123, 456];
#[cfg(unix)]
#[no_mangle]
pub static mut global_array_with_different_sizes: [i32; 1] = [7890];

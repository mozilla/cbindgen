#[cfg(all(unix, x11))]
#[repr(u32)]
enum FooType {
  A,
  B,
  C,
}

#[cfg(all(unix, x11))]
#[repr(C)]
struct FooHandle {
    ty: FooType,
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

#[cfg(any(windows, target_pointer_width="32"))]
#[repr(C)]
struct BarHandle {
    ty: BarType,
    x: i32,
    y: f32,
}

#[cfg(all(unix, x11))]
#[no_mangle]
pub extern "C" fn root(a: FooHandle)
{ }

#[cfg(any(windows, target_pointer_width="32"))]
#[no_mangle]
pub extern "C" fn root(a: BarHandle)
{ }

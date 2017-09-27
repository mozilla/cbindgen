#[cfg(windows)]
#[repr(C)]
struct Foo {
    x: f32,
}

#[cfg(unix)]
#[repr(C)]
struct Foo {
    y: f32,
}

#[no_mangle]
extern "C" fn root(a: Foo)
{ }

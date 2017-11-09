#[cfg(any(windows, unix))]
#[repr(C)]
struct Foo {
    x: i32,
}

#[cfg(windows)]
#[repr(C)]
struct Bar {
    y: Foo,
}

#[cfg(unix)]
#[repr(C)]
struct Bar {
    z: Foo,
}

#[repr(C)]
struct Root {
    w: Bar,
}

#[no_mangle]
extern "C" fn root(a: Root)
{ }

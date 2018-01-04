
struct Opaque();

#[repr(C)]
struct Foo {
    x: Option<&Opaque>,
    y: Option<&mut Opaque>,
    z: Option<fn () -> ()>,
}

#[repr(C)]
union Bar {
    x: Option<&Opaque>,
    y: Option<&mut Opaque>,
    z: Option<fn () -> ()>,
}

#[no_mangle]
pub extern "C" fn root(
	a: Option<&Opaque>,
    b: Option<&mut Opaque>,
    c: Foo,
    d: Bar
) { }

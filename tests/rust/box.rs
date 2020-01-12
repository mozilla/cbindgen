struct Opaque;

#[repr(C)]
pub struct Foo<T> {
    a: Box<f32>,
    b: Box<T>,
    c: Box<Opaque>,
    d: Box<Box<T>>,
    e: Box<Box<f32>>,
    f: Box<Box<Opaque>>,
    g: Option<Box<T>>,
    h: Option<Box<i32>>,
}

#[no_mangle]
pub extern "C" fn root(arg: Box<i32>, foo: *mut Foo<u64>, d: Box<Box<Opaque>>) {}

#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[repr(u8)]
pub enum Foo<T> {
    Foo { x: i32, y: Point<T>, z: Point<f32>, },
    Bar(T),
    Baz(Point<T>),
    Bazz,
}

#[no_mangle]
pub extern "C" fn foo(foo: *const Foo<i32>) {}

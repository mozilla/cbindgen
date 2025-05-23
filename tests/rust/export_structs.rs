#[repr(C)]
struct Foo {
    x: i32,
    y: f32,
}

#[repr(C)]
struct Bar {
    data: Foo,
}

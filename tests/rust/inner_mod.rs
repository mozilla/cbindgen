mod foo {
    #[repr(C)]
    struct Foo {
        x: f32,
    }
}

#[no_mangle]
extern "C" fn root(a: foo::Foo)
{ }

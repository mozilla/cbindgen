#[repr(C)]
pub struct Foo {
    a: bool,
    b: i32,
}

#[repr(u8)]
pub enum Bar {
    Baz,
    Bazz { named: Foo },
    FooNamed { different: i32, fields: u32 },
    FooParen(i32, Foo),
}

#[no_mangle]
pub extern "C" fn root(bar: Bar) -> Foo {
    unimplemented!();
}

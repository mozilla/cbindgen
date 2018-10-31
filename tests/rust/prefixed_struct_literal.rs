#[repr(C)]
struct Foo {
    a: i32,
    b: u32,
}

impl Foo {
    const FOO: Foo = Foo{ a: 42, b: 47, };
}

const BAR: Foo = Foo{ a: 42, b: 1337, };

#[no_mangle]
pub extern "C" fn root(x: Foo) { }

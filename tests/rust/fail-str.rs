#[repr(C)]
struct Foo<'a> {
    x: &'a str,
}

const BAR: &'static str = "";

#[no_mangle]
extern "C" fn root(a: Foo<'static>,
                   b: &'static str)
{ }

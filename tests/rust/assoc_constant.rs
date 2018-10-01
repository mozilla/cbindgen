#[repr(C)]
struct Foo {}

impl Foo {
    const GA: i32 = 10;
    const BU: &'static str = "hello world";
    const ZO: f32 = 3.14;
}

#[no_mangle]
pub extern "C" fn root(x: Foo) { }

/// cbindgen:rename-associated-constant=UpperCase
#[repr(C)]
struct Foo {}

impl Foo {
    pub const GA: i32 = 10;
    pub const ZO: f32 = 3.14;
}

#[no_mangle]
pub extern "C" fn root(x: Foo) { }


#[repr(C)]
pub struct MyFancyStruct {
    i: i32,
}

#[repr(C)]
pub enum MyFancyEnum {
    Foo,
    Bar(i32),
    Baz(i32),
}

#[repr(C)]
pub enum MyCLikeEnum {
    Foo1,
    Bar1,
    Baz1,
}

#[repr(C)]
pub union MyUnion {
    pub f: f32,
    pub u: u32,
}

#[no_mangle]
pub extern "C" fn root(s: MyFancyStruct, e: MyFancyEnum, c: MyCLikeEnum, u: MyUnion) {}

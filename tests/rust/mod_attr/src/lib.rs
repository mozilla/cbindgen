#[cfg(foo)]
pub const FOO: i32 = 1;

#[cfg(foo)]
#[no_mangle]
pub unsafe extern "C" fn foo(foo: &Foo) {}

#[cfg(foo)]
#[repr(C)]
pub struct Foo {}

#[cfg(feature = "foobar")]
pub mod foo {
    #[cfg(bar)]
    pub const BAR: i32 = 2;

    #[cfg(bar)]
    #[no_mangle]
    pub unsafe extern "C" fn bar(bar: &Bar) {}

    #[cfg(bar)]
    #[repr(C)]
    pub struct Bar {}
}

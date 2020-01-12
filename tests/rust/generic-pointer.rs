#[repr(C)]
struct Foo<T> {
    a: T,
}

type Boo<'a> = Foo<&'a u8>;

#[no_mangle]
pub extern "C" fn root(
    x: Boo,
) { }

#[repr(C)]
pub struct Foo<T> {
    a: T,
}

pub type Boo = Foo<u8>;

/// cbindgen:prefix-with-name=true
#[repr(C)]
pub enum Bar {
    Some,
    Thing,
}

/// cbindgen:prefix-with-name=true
#[repr(C)]
pub enum Dog<T> {
    Woof(T),
}

#[no_mangle]
pub extern "C" fn root(x: Boo, y: Bar, z: Dog<Foo<u8>>) {}

#[repr(C)]
pub struct MyStruct {
    number: std::mem::MaybeUninit<&i32>,
}

pub struct NotReprC<T> {
    inner: T,
}

pub type Foo = NotReprC<std::mem::MaybeUninit<&i32>>;

#[no_mangle]
pub extern "C" fn root(a: &Foo, with_maybe_uninit: &MyStruct) {}

#[repr(C)]
struct A<'a> {
    data: &'a i32
}

#[no_mangle]
pub extern "C" fn root<'a>(_a: A<'a>)
{ }

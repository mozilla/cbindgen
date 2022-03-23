#[repr(C)]
pub struct MyStruct<T> {
    int_field: u32,
    generic_field: T,
}

#[no_mangle]
pub extern "C" fn my_test() -> MyStruct<()> {
    MyStruct {
        int_field: 0,
        generic_field: ()
    }
}

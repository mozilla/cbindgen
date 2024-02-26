#[repr(C)]
pub enum COption<T> {
    Some(T),
    None,
}

#[no_mangle]
pub extern "C" fn root(a: COption<u8>, b: COption<u32>) {}

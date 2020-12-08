pub struct StructWithConstGeneric<const N: u8>;

pub const SOME_NUMBER: u8 = 20;

#[no_mangle]
pub extern "C" fn root(a: &StructWithConstGeneric<SOME_NUMBER>) {}

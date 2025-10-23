pub type Type1 = [i32; 3];
pub type Type2 = [i32; 15];

#[repr(C)]
pub struct Struct1 {
    id: usize,
}

#[repr(C)]
pub struct Struct2 {
    id: usize,
}

#[no_mangle]
pub extern "C" fn caller(s1: Struct1, s2: Struct2, t1: Type1, t2: Type2) {}

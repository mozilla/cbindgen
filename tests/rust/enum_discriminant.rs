pub const FOUR: i8 = 4;

#[repr(i8)]
enum E {
    A = 1,
    B = -1,
    C = 1 + 2,
    D = FOUR,
    F = (5),
}

#[repr(i8)]
enum E_NoCython {
    G = '6' as i8,   // Not supported
    H = false as i8, // Not supported
}

#[no_mangle]
pub extern "C" fn root(_: &E) {}

#[no_mangle]
pub extern "C" fn root_no_cython(_: &E_NoCython) {}

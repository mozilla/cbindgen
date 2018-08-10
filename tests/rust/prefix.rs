const LEN: i32 = 42;

pub type NamedLenArray = [i32; LEN];
pub type ValuedLenArray = [i32; 42];

#[no_mangle]
pub extern "C" fn root(x: NamedLenArray, y: ValuedLenArray) { }

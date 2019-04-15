pub const LEN: i32 = 42;

pub type NamedLenArray = [i32; LEN];
pub type ValuedLenArray = [i32; 42];

#[repr(u8)]
pub enum AbsoluteFontWeight {
    Weight(f32),
    Normal,
    Bold,
}

#[no_mangle]
pub extern "C" fn root(x: NamedLenArray, y: ValuedLenArray, z: AbsoluteFontWeight) { }

pub const SMALL: u8 = 255;
pub const LARGE: u64 = 18446744073709551615;
pub const NEGATIVE: i32 = -42;
pub const MASK: u32 = !0u32;
pub const ARITHMETIC: usize = (4 + 2) * 3;
pub const SHIFT: u32 = 1u32 << 31;
pub const BITWISE: u32 = 5 | 2;
pub const ENABLED: bool = true;
pub const CHARACTER: char = 'x';
pub const FLOAT: f32 = 0.1;
pub const DOUBLE: f64 = 0.3333333333333333;
pub const FLOAT_DIVISION: f64 = 1.0 / 3.0;
pub const NESTED_NEGATION: i64 = -(!42);
pub const BOUND: u32 = u32::MAX;
pub const SIGNED_MIN: i32 = i32::MIN;
pub const POINTER: *const u8 = 1 as *const u8;
pub const ARRAY: [u8; 2] = [1, 2];
pub const STRING: &str = "text";

#[repr(C)]
pub struct Aggregate {
    pub value: u32,
}

pub const AGGREGATE: Aggregate = Aggregate { value: 4 };
pub type Alias = u32;
pub const ALIAS: Alias = 5;

impl Aggregate {
    pub const ASSOCIATED: u32 = 6;
}

#[no_mangle]
pub extern "C" fn use_types(_: Aggregate, _: Alias) {}

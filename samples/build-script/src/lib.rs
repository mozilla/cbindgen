#![allow(dead_code)]
#![allow(unused_variables)]

#[repr(u32)]
pub enum Options {
    First,
    Second,
    LastOne,
}

pub struct Opaque {
    x: i32,
    y: f32,
}

/// cbindgen:rename-all=UpperCase
#[repr(C)]
pub struct Normal {
    x: i32,
    y: f32,
}

#[repr(C)]
pub struct Comparable {
    x: i32,
}

#[repr(C)]
pub struct TupleStruct(i32, f32);

#[no_mangle]
pub extern "C" fn root(x: *mut Opaque,
                       y: Normal,
                       z: Comparable,
                       w: Options,
                       a: TupleStruct)
{

}

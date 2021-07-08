struct StructAbc {
    x: i32,
    y: f32,
}

#[repr(C)]
struct StructDef {
    x: i32,
    y: f32,
}

union UnionGhi {
    x: i32,
    y: f32,
}

#[repr(C)]
union UnionJkl {
    x: i32,
    y: f32,
}

#[repr(u8)]
enum Enumeration {
    x = 0,
    y = 1,
}

type TypeAlias = StructAbc;

#[no_mangle]
pub static StaticAbc: i32 = 10;

pub const ConstantAbc: i32 = 10;

pub const ConstantExpression: isize = 10 as *mut TypeAlias as isize;

#[no_mangle]
pub extern "C" fn root(
    a: *const StructAbc,
    b: StructDef,
    c: UnionGhi,
    d: UnionJkl,
    e: Enumeration,
    f: TypeAlias,
) { }


#[repr(C)]
pub struct UnusedStruct {
    x: i32,
    y: f32,
}

#[repr(u8)]
pub enum UnusedEnum {
    A,
    B,
}

#[repr(transparent)]
pub struct UnusedTransparent(u32);

#[repr(C)]
pub union UnusedUnion {
    x: i32,
    y: f32,
}

pub type UnusedAlias = i32;

pub struct NotCbindgenable {
    x: i32,
}

pub struct ExplicitOpaque {
    x: i32,
}

struct OpaqueDependency {
    x: i32,
}

#[repr(C)]
pub struct UsesOpaqueDependency {
    opaque: *mut OpaqueDependency,
}

#[repr(C)]
pub struct ExcludedStruct {
    x: i32,
}

#[repr(C)]
struct PrivateStruct {
    x: i32,
}

#[repr(u8)]
enum PrivateEnum {
    A,
}

#[repr(transparent)]
struct PrivateTransparent(u32);

#[repr(C)]
union PrivateUnion {
    x: i32,
}

type PrivateAlias = i32;

mod private_module {
    #[repr(C)]
    pub struct PublicInPrivateModule {
        pub x: i32,
    }
}

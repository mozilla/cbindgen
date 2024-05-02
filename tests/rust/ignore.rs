/// cbindgen:ignore
#[no_mangle]
pub extern "C" fn root() {}

/// cbindgen:ignore
///
/// Something else.
#[no_mangle]
pub extern "C" fn another_root() {}

#[no_mangle]
pub extern "C" fn no_ignore_root() {}

/// cbindgen:ignore
#[repr(C)]
pub struct IgnoredStruct {}

pub struct IgnoredStructImpl;

/// cbindgen:ignore
impl IgnoredStructImpl {}

/// cbindgen:ignore
pub const IGNORED_CONST: u32 = 0;

pub const NOT_IGNORED_CONST: u32 = 0;

pub struct StructWithIgnoredImplMembers;

impl StructWithIgnoredImplMembers {
    /// cbindgen:ignore
    #[no_mangle]
    pub extern "C" fn ignored_associated_method() {}

    #[no_mangle]
    pub extern "C" fn no_ignore_associated_method() {}

    /// cbindgen:ignore
    pub const IGNORED_INNER_CONST: u32 = 0;

    pub const NOT_IGNORED_INNER_CONST: u32 = 0;
}

/// cbindgen:ignore
enum IgnoredEnum {}

enum NotIgnoredEnum {}

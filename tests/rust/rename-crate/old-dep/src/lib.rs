#[repr(C)]
pub struct RenamedTy {
    y: u64,
}

#[cfg(not(target_os = "freebsd"))]
#[repr(C)]
pub struct ContainsNoExternTy {
    pub field: no_extern::NoExternTy,
}

#[cfg(target_os = "freebsd")]
#[repr(C)]
pub struct ContainsNoExternTy {
    pub field: u64,
}

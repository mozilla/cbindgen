#[allow(unexpected_cfgs)]
#[no_mangle]
pub extern "C" fn cfg_function_args(
    first: u32,
    #[cfg(feature = "enabled")] enabled: u32,
    #[cfg(feature = "disabled")] disabled: u32,
    last: u32,
) {
}

#[allow(unexpected_cfgs)]
#[no_mangle]
pub extern "C" fn cfg_first_argument(
    #[cfg(feature = "enabled")] enabled: u32,
    last: u32,
) {
}

#[allow(unexpected_cfgs)]
#[no_mangle]
pub extern "C" fn cfg_all_arguments(
    #[cfg(feature = "enabled")] enabled: u32,
    #[cfg(feature = "disabled")] disabled: u32,
) {
}

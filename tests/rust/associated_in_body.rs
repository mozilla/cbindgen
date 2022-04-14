bitflags! {
    /// Constants shared by multiple CSS Box Alignment properties
    ///
    /// These constants match Gecko's `NS_STYLE_ALIGN_*` constants.
    #[derive(MallocSizeOf)]
    #[repr(C)]
    pub struct AlignFlags: u8 {
        /// 'auto'
        const AUTO = 0;
        /// 'normal'
        const NORMAL = 1;
        /// 'start'
        const START = 1 << 1;
        /// 'end'
        const END = 1 << 2;
        const ALIAS = Self::END.bits;
        /// 'flex-start'
        const FLEX_START = 1 << 3;
        const MIXED = 1 << 4 | AlignFlags::FLEX_START.bits | AlignFlags::END.bits;
        const MIXED_SELF = 1 << 5 | AlignFlags::FLEX_START.bits | AlignFlags::END.bits;
    }
}

#[no_mangle]
pub extern "C" fn root(flags: AlignFlags) {}

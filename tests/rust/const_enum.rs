#[repr(u8)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

pub const DEFAULT_FILL_RULE: FillRule = FillRule::Nonzero;

pub const ALL_FILL_RULES: [FillRule; 2] = [FillRule::Nonzero, FillRule::Evenodd];

#[repr(C)]
pub struct Style {
    rule: FillRule,
}

impl Style {
    pub const DEFAULT_RULE: FillRule = FillRule::Nonzero;
    pub const ALL_RULES: [FillRule; 2] = [FillRule::Nonzero, FillRule::Evenodd];
}

#[no_mangle]
pub extern "C" fn root(rule: FillRule, style: Style) {}

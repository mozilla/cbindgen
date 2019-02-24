#[repr(transparent)]
struct Transparent { field: u8 }

const FOO: Transparent = Transparent { field: 0 };

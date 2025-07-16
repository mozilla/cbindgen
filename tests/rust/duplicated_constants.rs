struct Foo {
    field: u32,
}

impl Foo {
    pub const FIELD_RELATED_CONSTANT: u32 = 0;
}

struct Bar {
    field: u32,
}

impl Bar {
    pub const FIELD_RELATED_CONSTANT: u32 = 0;
}

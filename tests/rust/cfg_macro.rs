#[repr(C)]
pub struct Collision {
    #[cfg(all(a, b_c))]
    pub x: i32,
    #[cfg(all(a_b, c))]
    pub y: i32,
}

#[repr(C)]
pub struct Nested {
    pub collision: Collision,
    pub pointer: *const u8,
}

pub const NESTED: Nested = Nested {
    collision: Collision {
        #[cfg(all(a, b_c))]
        x: 1,
        #[cfg(all(a_b, c))]
        y: 2,
    },
    pointer: 0 as *const u8,
};

#[repr(C)]
pub struct Foo {
    #[cfg(windows)]
    pub x: i32,
}

pub const FOO: Foo = Foo {
    #[cfg(windows)]
    x: 0,
};

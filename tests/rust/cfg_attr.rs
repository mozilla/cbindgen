#[cfg_attr(feature = "cffi", repr(C))]
struct Normal {
    x: i32,
    y: f32,
}

/// cbindgen:rename-all=GeckoCase
#[cfg_attr(feature = "cffi", repr(C))]
struct TupleRenamed(i32, f32);

/// cbindgen:field-names=[x, y]
#[cfg_attr(feature = "cffi", repr(C))]
struct TupleNamed(i32, f32);

#[cfg_attr(feature = "cffi", no_mangle)]
pub extern "C" fn root(
    b: Normal,
    d: TupleRenamed,
    e: TupleNamed
) { }

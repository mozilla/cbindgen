struct Opaque {
    x: i32,
    y: f32,
}

#[repr(C)]
struct Normal {
    x: i32,
    y: f32,
}

#[repr(C)]
struct Tuple(i32, f32);

/// cbindgen:field-names=[x, y]
#[repr(C)]
struct TupleNamed(i32, f32);

#[no_mangle]
extern "C" fn root(x: *mut Opaque, y: Normal, z: Tuple, w: TupleNamed)
{

}

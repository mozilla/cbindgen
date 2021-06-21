type DoFn = extern "C" fn(x: i32, y: i32) -> i32;
type NonNullAlias<T> = std::ptr::NonNull<T>;

#[repr(C)]
struct StructWithOptionalFunctionPointer {
    func: DoFn,
    maybe_func: Option<DoFn>,
}

#[repr(C)]
struct StructWithOptionalNonNullPointer {
    data: NonNullAlias<u32>,
    maybe_data: Option<NonNullAlias<u32>>,
}

#[no_mangle]
pub extern "C" fn root(
    swofp: StructWithOptionalFunctionPointer,
    swonnp: StructWithOptionalNonNullPointer,
) {
}

/// Regression test: in C++ output, the generated static variant constructors of a
/// tagged enum must value-initialize their `result` local (`Foo result{};`) so that
/// the payload union is zero-initialized rather than left indeterminate. This avoids
/// false "uninitialized scalar" reports from static analysis for no-payload variants.
///
/// cbindgen:derive-helper-methods=true
#[repr(C, u8)]
pub enum ValueInitEnum {
    /// No payload: `result` is returned with only the tag set, so the union must be
    /// value-initialized.
    Empty,
    /// Payload variant: constructed via placement new over the value-initialized union.
    Number(i32),
    Pair(i32, u64),
}

#[no_mangle]
pub extern "C" fn root(e: ValueInitEnum) {}

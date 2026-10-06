pub type A<T> = T;
pub type B<T> = [T; 2];
pub type C<T> = extern "C" fn(T, u8);
pub type D<T> = *const [T; 2];

/* 
    This test would produce bindings that won't compile:
    - in C, the aliases would be unresolved,
    - in C++, compiler would eagerly detect ill-formed C and D.
    If you make the function below `pub extern 'C'` and manually run cbindgen on it,
    the warnings produced for it would match the following annotations:

cbindgen:test-expect-warning-cpp=Skipping a typedef alias A_1Z of a zero sized type
cbindgen:test-expect-warning-c=Skipping a typedef alias A_1Z of a zero sized type
cbindgen:test-expect-warning-cpp=Skipping a typedef alias B_1Z of a zero sized type
cbindgen:test-expect-warning-c=Skipping a typedef alias B_1Z of a zero sized type
cbindgen:test-expect-warning-cpp=C++ bindings for C (instantiated as C_1Z) may be ill-formed
cbindgen:test-expect-warning-cpp=C++ bindings for D (instantiated as D_1Z) may be ill-formed
*/
#[no_mangle]
/* pub extern "C" */ fn use_typedefs_SKIPPED(a: A<()>, b: B<()>, c: C<()>, d: D<()>) {}

// placing C and D under a pointer over template postpones their instantiation,
// so we can check the warnings while keeping bindings compilable
#[repr(C)]
pub struct Wrapper<T> { c: C<T>, d: D<T>, }


/// cbindgen:test-expect-warning-cpp=C++ bindings for C (instantiated as C_1Z) may be ill-formed
/// cbindgen:test-expect-warning-cpp=C++ bindings for D (instantiated as D_1Z) may be ill-formed
#[no_mangle]
pub extern "C" fn use_typedefs_ptr(w: *const Wrapper<()>) {}

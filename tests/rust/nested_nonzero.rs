#[repr(transparent)]
struct Handle<T> {
    value: std::num::NonZeroU32,
    _phantom: std::marker::PhantomData<T>,
}

struct File;

#[repr(C)]
struct Node {
    file: Handle<File>,
    maybe_file: Option<Handle<File>>,
}

#[no_mangle]
pub extern "C" fn root(node: &Node) {}

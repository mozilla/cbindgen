#[repr(C)]
struct Foo<T> {
    data: T
}

struct Bar<T> {
    data: T
}

#[no_mangle]
extern "C" fn root(a: Foo<i32>, b: Foo<f32>, c: Bar<f32>) {
}

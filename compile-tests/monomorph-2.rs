#[repr(C)]
struct List<T> {
     members: *mut T,
     count: usize
}

struct A;

struct B;

#[no_mangle]
extern "C" fn foo(a: List<A>) { }

#[no_mangle]
extern "C" fn bar(b: List<B>) { }

#[repr(C)]
struct Foo<T> {
    x: T,
}

#[repr(C)]
struct NotReturnValue<T> {
    x: T,
}

#[repr(C)]
struct FooField {
    f: extern "C" fn() -> Foo<i8>,
    g: extern "C" fn(NotReturnValue<i32>),
}

#[repr(C)]
struct Bar<P, Q> {
    p: P,
    q: Q,
}

#[repr(transparent)]
struct Transparent {
    x: Foo<i64>,
}

pub type IntBar<T> = Bar<i8, T>;

pub type IntBoolBar = IntBar<bool>;

pub type WrapFoo<T> = Foo<T>;

pub type BoolBoolBar = Bar<bool, bool>;

pub type WrapBoolBoolBar = BoolBoolBar;

pub type WrapNonZeroInt = NonZero<i8>;

// Negatie case: Not generic
#[no_mangle]
pub extern "C" fn fnA() -> i32 { todo!() }

// Negative case: Transparent and underlying is not a monomorph
#[no_mangle]
pub extern "C" fn fnB() -> NonZero<i16> { todo!() }

// Negative case: cbindgen does not support template functions in the first place
#[no_mangle]
pub extern "C" fn fnC<T>() -> Foo<T> { todo!() }

// Negative case: Not emitted because opaque, but anyway would fail to compile because Option<T> only has
// a forward declaration.
//#[no_mangle]
//pub extern "C" fn fnD() -> Option<i32> { todo!() }

#[no_mangle]
pub extern "C" fn fnE() -> Foo<i16> { todo!() }

#[no_mangle]
pub extern "C" fn fnF(f: FooField) {}

#[no_mangle]
pub extern "C" fn fnG() -> Bar<i16, i16> { todo!() }

#[no_mangle]
pub extern "C" fn fnH() -> IntBar<i32> { todo!() }

#[no_mangle]
pub extern "C" fn fnI() -> IntBoolBar { todo!() }

#[no_mangle]
pub extern "C" fn fnJ() -> WrapFoo<i32> { todo!() }

#[no_mangle]
pub extern "C" fn fnK() -> WrapBoolBoolBar { todo!() }

#[no_mangle]
pub extern "C" fn fnL() -> Foo<bool> { todo!() }

// Negative case: transparent and underlying is not a template type
#[no_mangle]
pub extern "C" fn fnM() -> WrapNonZeroInt { todo!() }

#[no_mangle]
pub extern "C" fn fnN() -> Transparent { todo!() }

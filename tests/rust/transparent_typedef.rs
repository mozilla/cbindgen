/// cbindgen:transparent-typedef
#[repr(transparent)]
pub struct Transparent<T, P=c_void> {
    field: T,
    _phantom: std::marker::PhantomData<P>,
}

/// cbindgen:transparent-typedef
pub type Alias<T> = Transparent<T>;


#[repr(C)]
pub struct AlwaysErased1<T> {
    a: Alias<T>,
    n: NonNull<T>,
    t: Transparent<T>,
}

// Option<T> only gets erased if T is NonNull or NonZero; use references so it still compiles.
#[repr(C)]
pub struct SometimesErased1<T> {
    o: &Option<T>,
}

#[no_mangle]
pub extern "C" fn root1(
    a: AlwaysErased1<i32>,
    sn: SometimesErased1<NonNull<i16>>,
    sz: SometimesErased1<NonZero<i32>>,
    si: SometimesErased1<i64>) {}

#[repr(C)]
pub struct AlwaysErased2<T> {
    aa: Alias<Alias<T>>,
    an: Alias<NonNull<T>>,
    at: Alias<Transparent<T>>,
    na: NonNull<Alias<T>>,
    nn: NonNull<NonNull<T>>,
    nt: NonNull<Transparent<T>>,
    on: Option<NonNull<T>>,
    ta: Transparent<Alias<T>>,
    tn: Transparent<NonNull<T>>,
    tt: Transparent<Transparent<T>>,
}

// Option<T> only gets erased if T is NonNull or NonZero; use references so it still compiles.
#[repr(C)]
pub struct SometimesErased2<T> {
    ao: &Alias<Option<T>>,
    no: &NonNull<Option<T>>,
    oa: &Option<Alias<T>>,
    ot: &Option<Transparent<T>>,
    to: &Transparent<Option<T>>,
}

// Use references so it still compiles.
#[repr(C)]
pub struct NeverErased2<T> {
    oo: &Option<Option<T>>,
}

#[no_mangle]
pub extern "C" fn root2(
    a: AlwaysErased2<i32>,
    sn: SometimesErased2<NonNull<i16>>,
    sz: SometimesErased2<NonZero<i32>>,
    si: SometimesErased2<i64>,
    n: NeverErased2<i32>) {}

#[repr(C)]
pub struct AlwaysErasedMany<T> {
    // A few erasable quadruplets
    tont: Transparent<Option<NonNull<Transparent<T>>>>,
    otnt: Option<Transparent<NonNull<Transparent<T>>>>,
    totn: Transparent<Option<Transparent<NonNull<T>>>>,

    // One erasable quintuplet
    totnt: Transparent<Option<Transparent<NonNull<Transparent<T>>>>>,
}

#[no_mangle]
pub extern "C" fn root_many(a: AlwaysErasedMany<i32>) {}

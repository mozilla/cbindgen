use std::num::NonZero;
use std::ptr::NonNull;

pub struct Opaque<U = Alias<i32>>;

#[no_mangle]
pub extern "C" fn root_opaque(o: Opaque<Alias<Option<NonZero<i32>>>>) {}

#[repr(C)]
pub struct Struct<U = Alias<U>> {
    field: U,
}

/// cbindgen:transparent-typedef
#[repr(transparent)]
pub struct Transparent<T, P=T> {
    field: T,
    _phantom: std::marker::PhantomData<P>,
}

pub type Typedef<U = Transparent<i64>> = Transparent<U>;

/// cbindgen:transparent-typedef
pub type Alias<U = Transparent<i64>> = Transparent<U>;

#[repr(C)]
pub struct FullyTransparent1<E = Alias<i64>> {
    a: Alias<E>,
    n: NonNull<E>,
    t: Transparent<E>,
    f: extern "C" fn(a: Alias<E>, n: NonNull<E>) -> Transparent<E>,

    ai: Alias<i32>,
    ni: NonNull<i32>,
    ti: Transparent<i32>,
    fi: extern "C" fn(ai: Alias<i32>, ni: NonNull<i32>) -> Transparent<i32>,
}

// Option<T> only gets erased if T is NonNull or NonZero; use references so it still compiles.
#[repr(C)]
pub struct NotTransparent1<'a, E = Option<i64>> {
    o: &'a Option<E>,
    s: &'a Struct<E>,
    t: &'a Typedef<E>,
    f: extern "C" fn(o: &Option<E>, s: &Struct<E>) -> *mut Typedef<E>,

    oi: &'a Option<i32>,
    si: &'a Struct<i32>,
    ti: &'a Typedef<i32>,
    fi: extern "C" fn(oi: &Option<i32>, si: &Struct<i32>) -> *mut Typedef<i32>,
}

#[no_mangle]
pub extern "C" fn test1(
    a: Struct<i32>,
    b: Typedef<i32>,
    c: &Option<i32>,
    d: Transparent<i32>,
    e: Alias<i32>,
) {}

#[no_mangle]
pub extern "C" fn root1(
    a: FullyTransparent1<Alias<i32>>,
    s: NotTransparent1<Option<i32>>) {}

#[repr(C)]
pub struct FullyTransparent2<E = Option<NonZero<i32>>> {
    aa: Alias<Alias<E>>,
    an: Alias<NonNull<E>>,
    at: Alias<Transparent<E>>,
    na: NonNull<Alias<E>>,
    nn: NonNull<NonNull<E>>,
    nt: NonNull<Transparent<E>>,
    on: Option<NonNull<E>>,
    ta: Transparent<Alias<E>>,
    tn: Transparent<NonNull<E>>,
    tt: Transparent<Transparent<E>>,
    f: extern "C" fn(
        aa: Alias<Alias<E>>,
        an: Alias<NonNull<E>>,
        at: Alias<Transparent<E>>,
        na: NonNull<Alias<E>>,
        nn: NonNull<NonNull<E>>,
        nt: NonNull<Transparent<E>>,
        on: Option<NonNull<E>>,
        ta: Transparent<Alias<E>>,
        tn: Transparent<NonNull<E>>,
    ) -> Transparent<Transparent<E>>,

    aai: Alias<Alias<i32>>,
    ani: Alias<NonNull<i32>>,
    ati: Alias<Transparent<i32>>,
    nai: NonNull<Alias<i32>>,
    nni: NonNull<NonNull<i32>>,
    nti: NonNull<Transparent<i32>>,
    oni: Option<NonNull<i32>>,
    tai: Transparent<Alias<i32>>,
    tni: Transparent<NonNull<i32>>,
    tti: Transparent<Transparent<i32>>,
    fi: extern "C" fn(
        aai: Alias<Alias<i32>>,
        ani: Alias<NonNull<i32>>,
        ati: Alias<Transparent<i32>>,
        nai: NonNull<Alias<i32>>,
        nni: NonNull<NonNull<i32>>,
        nti: NonNull<Transparent<i32>>,
        oni: Option<NonNull<i32>>,
        tai: Transparent<Alias<i32>>,
        tni: Transparent<NonNull<i32>>,
    ) -> Transparent<Transparent<i32>>,
}

// Option<E> only gets erased if T is NonNull or NonZero; use references so it still compiles.
#[repr(C)]
pub struct PartlyTransparent2<'a, E = Option<Alias<i64>>> {
    ao: &'a Alias<Option<E>>,
    aS: &'a Alias<Struct<E>>,
    at: &'a Alias<Typedef<E>>,
    no: &'a NonNull<Option<E>>,
    ns: &'a NonNull<Struct<E>>,
    nt: &'a NonNull<Typedef<E>>,
    f: extern "C" fn(
        ao: &Alias<Option<E>>,
        aS: &Alias<Struct<E>>,
        at: &Alias<Typedef<E>>,
        no: &NonNull<Option<E>>,
        ns: &NonNull<Struct<E>>,
    ) -> *mut NonNull<Typedef<E>>,

    aoi: &'a Alias<Option<i32>>,
    asi: &'a Alias<Struct<i32>>,
    ati: &'a Alias<Typedef<i32>>,
    noi: &'a NonNull<Option<i32>>,
    nsi: &'a NonNull<Struct<i32>>,
    nti: &'a NonNull<Typedef<i32>>,
    fi: extern "C" fn(
        aoi: &Alias<Option<i32>>,
        asi: &Alias<Struct<i32>>,
        ati: &Alias<Typedef<i32>>,
        noi: &NonNull<Option<i32>>,
        nsi: &NonNull<Struct<i32>>,
    ) -> *mut NonNull<Typedef<i32>>,
}

// Use references so it still compiles.
#[repr(C)]
pub struct NotTransparent2<'a, E = Option<Struct<i64>>> {
    oo: &'a Option<Option<E>>,
    os: &'a Option<Struct<E>>,
    so: &'a Struct<Option<E>>,
    ss: &'a Struct<Struct<E>>,
    f: extern "C" fn(
        oo: &Option<Option<E>>,
        os: &Option<Struct<E>>,
        so: &Struct<Option<E>>,
    ) -> *mut Struct<Struct<E>>,

    ooi: &'a Option<Option<i32>>,
    osi: &'a Option<Struct<i32>>,
    soi: &'a Struct<Option<i32>>,
    ssi: &'a Struct<Struct<i32>>,
    fi: extern "C" fn(
        ooi: &Option<Option<i32>>,
        osi: &Option<Struct<i32>>,
        soi: &Struct<Option<i32>>,
    ) -> *mut Struct<Struct<i32>>,
}

#[no_mangle]
pub extern "C" fn root2(
    a: FullyTransparent2<Transparent<Alias<i32>>>,
    s: PartlyTransparent2<Option<Alias<i32>>>,
    n: NotTransparent2<Struct<Option<i32>>>) {}

#[repr(C)]
pub enum FullyTransparentMany<E = Alias<Option<Transparent<NonZero<Transparent<i64>>>>>> {
    ont(Option<NonNull<Transparent<E>>>),
    otn(Option<Transparent<NonNull<E>>>),
    ton(Transparent<Option<NonNull<E>>>),

    // One erasable quadruple
    totn(Transparent<Option<Transparent<NonNull<E>>>>),

    f(extern "C" fn(
        ont: Option<NonNull<Transparent<E>>>,
        otn: Option<Transparent<NonNull<E>>>,
        ton: Transparent<Option<NonNull<E>>>,
    ) -> Transparent<Option<Transparent<NonNull<E>>>>),

    onti(Option<NonNull<Transparent<i32>>>),
    otni(Option<Transparent<NonNull<i32>>>),
    toni(Transparent<Option<NonNull<i32>>>),

    // One erasable quadruple
    totni(Transparent<Option<Transparent<NonNull<i32>>>>),

    fi(extern "C" fn(
        onti: Option<NonNull<Transparent<i32>>>,
        otni: Option<Transparent<NonNull<i32>>>,
        toni: Transparent<Option<NonNull<i32>>>,
    ) -> Transparent<Option<Transparent<NonNull<i32>>>>),

}

#[repr(C)]
pub union PartlyTransparentMany<'a, E = Transparent<Option<Alias<i64>>>> {
    // A few triples
    tao: &'a Transparent<Alias<Option<E>>>,
    toa: &'a Transparent<Option<Alias<E>>>,
    ota: &'a Option<Transparent<Alias<E>>>,
    tas: &'a Transparent<Alias<Struct<E>>>,
    tsa: &'a Transparent<Struct<Alias<E>>>,
    sta: &'a Struct<Transparent<Alias<E>>>,

    // Two quadruples
    toat: &'a Transparent<Option<Alias<Transparent<E>>>>,
    tsat: &'a Transparent<Struct<Alias<Transparent<E>>>>,

    // A few triples
    taoi: &'a Transparent<Alias<Option<i32>>>,
    toai: &'a Transparent<Option<Alias<i32>>>,
    otai: &'a Option<Transparent<Alias<i32>>>,
    tasi: &'a Transparent<Alias<Struct<i32>>>,
    tsai: &'a Transparent<Struct<Alias<i32>>>,
    stai: &'a Struct<Transparent<Alias<i32>>>,

    // Two quadruples
    toati: &'a Transparent<Option<Alias<Transparent<i32>>>>,
    tsati: &'a Transparent<Struct<Alias<Transparent<i32>>>>,
}

#[no_mangle]
pub extern "C" fn root_many(
    a: FullyTransparentMany<Alias<Option<NonNull<i32>>>>,
    b: PartlyTransparentMany<Alias<Option<Transparent<i32>>>>,
) {}

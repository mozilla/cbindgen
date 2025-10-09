pub trait Inner{
    type inner;
}

pub trait Middle{
    type middle;
}

pub trait Outer{
    type outer;
}

pub trait Double{
    type double;
}

impl Double for u32{
    type double = [<u32 as Outer>::outer; 10];
}

impl Outer for u32{
    type outer = <u32 as Middle>::middle;
}

impl Middle for u32{
    type middle = <u32 as Inner>::inner;
}

impl Inner for u32{
    type inner = bool;
}

#[no_mangle]
extern fn test_nested_assoc(out: <u32 as Outer>::outer, mid: <u32 as Middle>::middle) -> <u32 as Inner>::inner {}

#[no_mangle]
extern fn test_double_assoc(double: <u32 as Double>::double){}

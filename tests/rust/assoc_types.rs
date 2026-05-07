pub trait MyTrait{
    type SomeType;
}

pub trait AnotherTrait{
    type AnotherType;
}

#[repr(C)]
pub struct MyStruct{
    a: u8,
}

impl MyTrait for MyStruct{
    type SomeType = i64;
}

impl AnotherTrait for MyStruct{
    type AnotherType = [u8; 5];
}

impl MyTrait for i32{
    type SomeType = bool;
}

impl AnotherTrait for i32{
    type AnotherType = u64;
}

pub const CONST_TEST_1: <MyStruct as MyTrait>::SomeType = 50;
pub const CONST_TEST_2: <i32 as AnotherTrait>::AnotherType = 100;

#[no_mangle]
pub static STATIC_TEST_1: [<i32 as AnotherTrait>::AnotherType; 5] = [1, 2, 3, 4, 5];
#[no_mangle]
pub static STATIC_TEST_2: <i32 as MyTrait>::SomeType = false;

#[repr(C)]
pub enum EnumTest{
    enum_var1,
    enum_var2(<MyStruct as AnotherTrait>::AnotherType),
    enum_var3{
        a: <i32 as MyTrait>::SomeType,
        b: <i32 as AnotherTrait>::AnotherType,
    }
}

#[repr(C)]
pub struct AnotherStruct{
    a: u8,
    b: <MyStruct as MyTrait>::SomeType,
    c: [<i32 as MyTrait>::SomeType; 36],
}

#[repr(C)]
pub enum UnionTest{
    union_var1,
    union_var2(<MyStruct as AnotherTrait>::AnotherType),
    union_var3{
        a: <i32 as MyTrait>::SomeType,
        b: <i32 as AnotherTrait>::AnotherType,
    }
}

pub type typedef_test = <MyStruct as MyTrait>::SomeType;

#[no_mangle]
pub extern fn test_enum(enum_: EnumTest){}

#[no_mangle]
pub extern fn test_struct_gen(struct_: AnotherStruct){}

#[no_mangle]
pub extern fn test_union(union_: UnionTest){}

#[no_mangle]
pub extern fn test_typedefs(typedef: typedef_test){}

#[no_mangle]
pub extern fn test_fn(struct_: &<MyStruct as MyTrait>::SomeType) -> <MyStruct as MyTrait>::SomeType {
    struct_
}


#[no_mangle]
pub extern fn test_func_ptr(fn_ptr: extern fn(<i32 as AnotherTrait>::AnotherType, bool) -> <i32 as assoc::MyTrait>::SomeType){
}

#[no_mangle]
pub extern fn test_raw_ptr(a: *const <i32 as MyTrait>::SomeType, b: *mut <i32 as AnotherTrait>::AnotherType){}
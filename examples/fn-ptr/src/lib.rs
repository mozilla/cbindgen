type Callback = fn (i32, i32) -> bool;

#[no_mangle]
extern "C" fn root(x: fn(), y: Callback)
{

}

type A = fn ();
type B = fn (i32, i32) -> bool;
type C = fn (i32) -> fn (f32) -> bool;
type D = fn () -> *const [i32; 16];

type E = *const i32;
type F = *const *const i32;
type G = *const *mut i32;
type H = *const [i32; 16];
type I = *const fn (f32) -> f64;

type J = [i32; 16];
type K = [*const i32; 16];
type L = [fn (i32, i32) -> bool; 16];

#[no_mangle]
extern "C" fn root(a: A,
                   b: B,
                   c: C,
                   d: D,
                   e: E,
                   f: F,
                   g: G,
                   h: H,
                   i: I,
                   j: J,
                   k: K,
                   l: L)
{ }

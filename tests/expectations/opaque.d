module cbindgen;

@nogc nothrow @safe:

struct HashMap(K = void, V = void, Hasher = void);

struct Result(T = void, E = void);

/// Fast hash map used internally.
alias FastHashMap(K, V) = HashMap!(K, V, BuildHasherDefault!(DefaultHasher));

alias Foo = FastHashMap!(int, int);

alias Bar = Result!(Foo);

extern(C) {

void root(const Foo *a, const Bar *b);

}  // extern(C)

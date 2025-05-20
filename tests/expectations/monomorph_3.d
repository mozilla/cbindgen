module cbindgen;

@nogc nothrow @safe:

struct Bar(T = void);

union Foo(T) {
  const T *data;
}

union Tuple(T, E) {
  const T *a;
  const E *b;
}

alias Indirection(T) = Tuple!(T, float);

extern(C) {

void root(Foo!(int) a,
          Foo!(float) b,
          Bar!(float) c,
          Foo!(Bar!(float)) d,
          Bar!(Foo!(float)) e,
          Bar!(Bar!(float)) f,
          Tuple!(Foo!(float), float) g,
          Indirection!(float) h);

}  // extern(C)

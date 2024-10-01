module cbindgen;

@nogc nothrow @safe:

struct Example {
  @disable this();
  void (*f)(ulong, ulong);
}

extern(C) {

void loop_forever();

ubyte normal_return(Example arg, void (*other)(ubyte));

}  // extern(C)

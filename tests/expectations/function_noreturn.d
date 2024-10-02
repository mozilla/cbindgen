module cbindgen;

@nogc nothrow @safe:

struct Example {
  @disable this();
  void  function(ulong, ulong) f;
}

extern(C) {

void loop_forever();

ubyte normal_return(Example arg, void  function(ubyte) other);

}  // extern(C)

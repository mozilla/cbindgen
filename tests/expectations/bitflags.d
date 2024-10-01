module cbindgen;

@nogc nothrow @safe:

struct OutOfLine {
  @disable this();
  uint _0;
}

extern(C) {

void root(AlignFlags flags,
          DebugFlags bigger_flags,
          LargeFlags largest_flags,
          OutOfLine out_of_line);

}  // extern(C)

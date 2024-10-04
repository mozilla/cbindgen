module cbindgen;

@nogc nothrow @safe:

struct Option(T = void);

struct NonZeroAliases {
  @disable this();
  ubyte a;
  ushort b;
  uint c;
  ulong d;
  byte e;
  short f;
  int g;
  long h;
  long i;
  const Option!(long) *j;
}

struct NonZeroGenerics {
  @disable this();
  ubyte a;
  ushort b;
  uint c;
  ulong d;
  byte e;
  short f;
  int g;
  long h;
  long i;
  const Option!(long) *j;
}

extern(C) {

void root_nonzero_aliases(NonZeroAliases test,
                          ubyte a,
                          ushort b,
                          uint c,
                          ulong d,
                          byte e,
                          short f,
                          int g,
                          long h,
                          long i,
                          const Option!(long) *j);

void root_nonzero_generics(NonZeroGenerics test,
                           ubyte a,
                           ushort b,
                           uint c,
                           ulong d,
                           byte e,
                           short f,
                           int g,
                           long h,
                           long i,
                           const Option!(long) *j);

}  // extern(C)

module cbindgen;

@nogc nothrow @safe:

struct TakeUntil(uint V) {
  @disable this();
  const ubyte *start;
  ulong len;
  ulong point;
}

extern(C) {

TakeUntil!(0) until_nul(const ubyte *start, ulong len);

}  // extern(C)

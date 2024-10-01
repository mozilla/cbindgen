module cbindgen;

@nogc nothrow @safe:

struct Fns {
  @disable this();
  void (*noArgs)();
  void (*anonymousArg)(int);
  int (*returnsNumber)();
  byte (*namedArgs)(int first, short snd);
  byte (*namedArgsWildcards)(int _, short named, long _1);
}

extern(C) {

void root(Fns _fns);

void no_return();

}  // extern(C)

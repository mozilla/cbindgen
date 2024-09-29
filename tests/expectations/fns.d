module cbindgen;

@nogc nothrow @safe:

struct Fns {
  @disable this();
  void  function() noArgs;
  void  function(int) anonymousArg;
  int  function() returnsNumber;
  byte  function(int first, short snd) namedArgs;
  byte  function(int _, short named, long _1) namedArgsWildcards;
}

extern(C) {

void root(Fns _fns);

void no_return();

}  // extern(C)

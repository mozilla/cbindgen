#if 0
DEF DEFINE_FREEBSD = 0
#endif


module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();
  int x;
}

struct RenamedTy {
  @disable this();
  ulong y;
}

#if !defined(DEFINE_FREEBSD)
struct NoExternTy {
  @disable this();
  ubyte field;
}
#endif

#if !defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  @disable this();
  NoExternTy field;
}
#endif

#if defined(DEFINE_FREEBSD)
struct ContainsNoExternTy {
  @disable this();
  ulong field;
}
#endif

extern(C) {

void root(Foo a);

void renamed_func(RenamedTy a);

void no_extern_func(ContainsNoExternTy a);

}  // extern(C)

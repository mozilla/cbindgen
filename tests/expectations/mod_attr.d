#if 0
DEF FOO = 0
DEF BAR = 0
#endif


module cbindgen;

@nogc nothrow @safe:

#if defined(FOO)
enum FOO = 1;
#endif

#if defined(BAR)
enum BAR = 2;
#endif

#if defined(FOO)
struct Foo {
  @disable this();

}
#endif

#if defined(BAR)
struct Bar {
  @disable this();

}
#endif

extern(C) {

#if defined(FOO)
void foo(const Foo *foo);
#endif

#if defined(BAR)
void bar(const Bar *bar);
#endif

}  // extern(C)

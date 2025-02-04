/* Package version: 0.1.0 */

module cbindgen;

@nogc nothrow @safe:

struct Foo {
  @disable this();
  ulong bar;
}

extern(C) {

void doit(const Foo*);

}  // extern(C)

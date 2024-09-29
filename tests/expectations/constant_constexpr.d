module cbindgen;

@nogc nothrow @safe:

enum CONSTANT_I64 = 216;

enum CONSTANT_FLOAT32 = 312.292;

enum DELIMITER = ':';

enum LEFTCURLY = '{';

struct Foo {
  @disable this();
  int x;
}
enum Foo_CONSTANT_I64_BODY = 216;

enum SomeFoo = Foo(x: 99);

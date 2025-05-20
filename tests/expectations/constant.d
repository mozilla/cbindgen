module cbindgen;

@nogc nothrow @safe:

enum FOO = 10;

enum DELIMITER = ':';

enum LEFTCURLY = '{';

enum QUOTE = '\'';

enum TAB = '\t';

enum NEWLINE = '\n';

enum HEART = U'\U00002764';

enum EQUID = U'\U00010083';

enum ZOM = 3.14;

/// A single-line doc comment.
enum POS_ONE = 1;

/// A
/// multi-line
/// doc
/// comment.
enum NEG_ONE = -1;

enum SHIFT = 3;

enum XBOOL = 1;

enum XFALSE = ((0 << SHIFT) | XBOOL);

enum XTRUE = (1 << (SHIFT | XBOOL));

enum CAST = cast(ubyte)'A';

enum DOUBLE_CAST = cast(uint)cast(float)1;

struct Foo {
  @disable this();
  int [FOO] x;
}

extern(C) {

void root(Foo x);

}  // extern(C)

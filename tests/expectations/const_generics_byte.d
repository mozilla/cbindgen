module cbindgen;

@nogc nothrow @safe:

struct Parser(ubyte OPEN, ubyte CLOSE) {
  @disable this();
  ubyte *buf;
  ulong len;
}

extern(C) {

void init_parens_parser(Parser!(40, 41) *p, ubyte *buf, ulong len);

void destroy_parens_parser(Parser!(40, 41) *p);

void init_braces_parser(Parser!(123, 125) *p, ubyte *buf, ulong len);

}  // extern(C)

module cbindgen;

@nogc nothrow @safe:

(T = void)struct Option;

(T = void, E = void)struct Result;

struct String;

(T = void)struct Vec;

extern(C) {

void root(const Vec!(String) *a, const Option!(int) *b, const Result!(int, String) *c);

}  // extern(C)

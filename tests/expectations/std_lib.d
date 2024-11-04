module cbindgen;

@nogc nothrow @safe:

struct Option(T = void);

struct Result(T = void, E = void);

struct String;

struct Vec(T = void);

extern(C) {

void root(const Vec!(String) *a, const Option!(int) *b, const Result!(int, String) *c);

}  // extern(C)

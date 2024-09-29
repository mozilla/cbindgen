module cbindgen;

@nogc nothrow @safe:

extern(C) {

void test_camel_case(int fooBar);

void test_pascal_case(int FooBar);

void test_snake_case(int foo_bar);

void test_screaming_snake_case(int FOO_BAR);

void test_gecko_case(int aFooBar);

void test_prefix(int prefix_foo_bar);

}  // extern(C)

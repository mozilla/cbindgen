module cbindgen;

@nogc nothrow @safe:

alias MyCallback = void(*)(ulong a, ulong b);

alias MyOtherCallback = void(*)(ulong a, ulong lot, ulong of, ulong args, ulong and_then_some);

extern(C) {

void my_function(MyCallback a, MyOtherCallback b);

}  // extern(C)

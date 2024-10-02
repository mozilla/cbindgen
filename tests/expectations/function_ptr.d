module cbindgen;

@nogc nothrow @safe:

alias MyCallback = void function(ulong a, ulong b);

alias MyOtherCallback = void function(ulong a, ulong lot, ulong of, ulong args, ulong and_then_some);

extern(C) {

void my_function(MyCallback a, MyOtherCallback b);

}  // extern(C)

module cbindgen;

@nogc nothrow @safe:

enum IE : long {
  IV,
}

enum UE : ulong {
  UV,
}

alias Usize = ulong;

alias Isize = long;

extern(C) {

void root(Usize, Isize, UE, IE);

}  // extern(C)

module cbindgen;

@nogc nothrow @safe:

alias VaListFnPtr = int function(int count, ...);

alias VaListFnPtr2 = int function(int count, ...);

struct Interface(T) {
  @disable this();
  T fn1;
}

extern(C) {

int va_list_test(int count, ...);

int va_list_test2(int count, ...);

void va_list_fn_ptrs(int  function(int count, ...) fn1,
                     int  function(int count, ...) fn2,
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface!(int function(int count, ...)) fn5,
                     Interface!(int function(int count, ...)) fn6);

}  // extern(C)

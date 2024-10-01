module cbindgen;

@nogc nothrow @safe:

alias VaListFnPtr = int(*)(int count, ...);

alias VaListFnPtr2 = int(*)(int count, ...);

struct Interface(T) {
  @disable this();
  T fn1;
}

extern(C) {

int va_list_test(int count, ...);

int va_list_test2(int count, ...);

void va_list_fn_ptrs(int (*fn1)(int count, ...),
                     int (*fn2)(int count, ...),
                     VaListFnPtr fn3,
                     VaListFnPtr2 fn4,
                     Interface!(int(*)(int count, ...)) fn5,
                     Interface!(int(*)(int count, ...)) fn6);

}  // extern(C)

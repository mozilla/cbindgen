module cbindgen;

@nogc nothrow @safe:

alias A = void function();

alias B = void function();

alias C = bool function(int, int);

alias D = bool function function(int)(float);

alias E = const int([16] * function());

alias F = const int*;

alias G = const int*const *;

alias H = int*const *;

alias I = const int([16] *);

alias J = double function(float);

alias K = int[16] ;

alias L = const int*[16] ;

alias M = bool function[16] (int, int);

alias N = void function[16] (int, int);

alias P = void function(int named1st, bool, bool named3rd, int _);

extern(C) {

void  function() O();

void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);

}  // extern(C)

module cbindgen;

@nogc nothrow @safe:

alias A = void(*)();

alias B = void(*)();

alias C = bool(*)(int, int);

alias D = bool(*(*)(int))(float);

alias E = const int([16] *(*)());

alias F = const int*;

alias G = const int*const *;

alias H = int*const *;

alias I = const int([16] *);

alias J = double(**)(float);

alias K = int[16] ;

alias L = const int*[16] ;

alias M = bool(*[16] )(int, int);

alias N = void(*[16] )(int, int);

alias P = void(*)(int named1st, bool, bool named3rd, int _);

extern(C) {

void (*O())();

void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);

}  // extern(C)

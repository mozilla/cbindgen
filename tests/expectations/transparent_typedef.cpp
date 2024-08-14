#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Option;

template<typename T>
struct AlwaysErased1 {
  T a;
  T *n;
  T t;
};

template<typename T>
struct SometimesErased1 {
  const Option<T> *o;
};

template<typename T>
struct AlwaysErased2 {
  T aa;
  T *an;
  T at;
  T *na;
  T **nn;
  T *nt;
  T *on;
  T ta;
  T *tn;
  T tt;
};

template<typename T>
struct SometimesErased2 {
  const Option<T> *ao;
  Option<T> *const *no;
  const Option<T> *oa;
  const Option<T> *ot;
  const Option<T> *to;
};

template<typename T>
struct NeverErased2 {
  const Option<Option<T>> *oo;
};

template<typename T>
struct AlwaysErasedMany {
  T *tont;
  T *otnt;
  T *totn;
  T *totnt;
};

extern "C" {

void root1(AlwaysErased1<int32_t> a,
           SometimesErased1<int16_t*> sn,
           SometimesErased1<int32_t> sz,
           SometimesErased1<int64_t> si);

void root2(AlwaysErased2<int32_t> a,
           SometimesErased2<int16_t*> sn,
           SometimesErased2<int32_t> sz,
           SometimesErased2<int64_t> si,
           NeverErased2<int32_t> n);

void root_many(AlwaysErasedMany<int32_t> a);

}  // extern "C"

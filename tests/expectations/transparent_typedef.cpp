#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename U = int32_t>
struct Opaque;

template<typename T = void>
struct Option;

template<typename E = int64_t>
struct FullyTransparent1 {
  E a;
  E *n;
  E t;
  E (*f)(E a, E *n);
  int32_t ai;
  int32_t *ni;
  int32_t ti;
  int32_t (*fi)(int32_t ai, int32_t *ni);
};

template<typename U, typename P = U>
struct Struct {
  const U *field;
};

template<typename U = int64_t>
using Typedef = U;

template<typename E = Option<int64_t>>
struct NotTransparent1 {
  const Option<E> *o;
  const Struct<E> *s;
  const Typedef<E> *t;
  Typedef<E> *(*f)(const Option<E> *o, const Struct<E> *s);
  const Option<int32_t> *oi;
  const Struct<int32_t> *si;
  const Typedef<int32_t> *ti;
  Typedef<int32_t> *(*fi)(const Option<int32_t> *oi, const Struct<int32_t> *si);
};

template<typename E = int32_t>
struct FullyTransparent2 {
  E aa;
  E *an;
  E at;
  E *na;
  E **nn;
  E *nt;
  E *on;
  E ta;
  E *tn;
  E tt;
  E (*f)(E aa, E *an, E at, E *na, E **nn, E *nt, E *on, E ta, E *tn);
  int32_t aai;
  int32_t *ani;
  int32_t ati;
  int32_t *nai;
  int32_t **nni;
  int32_t *nti;
  int32_t *oni;
  int32_t tai;
  int32_t *tni;
  int32_t tti;
  int32_t (*fi)(int32_t aai,
                int32_t *ani,
                int32_t ati,
                int32_t *nai,
                int32_t **nni,
                int32_t *nti,
                int32_t *oni,
                int32_t tai,
                int32_t *tni);
};

template<typename E = Option<int64_t>>
struct PartlyTransparent2 {
  const Option<E> *ao;
  const Struct<E> *aS;
  const Typedef<E> *at;
  Option<E> *const *no;
  Struct<E> *const *ns;
  Typedef<E> *const *nt;
  Typedef<E> **(*f)(const Option<E> *ao,
                    const Struct<E> *aS,
                    const Typedef<E> *at,
                    Option<E> *const *no,
                    Struct<E> *const *ns);
  const Option<int32_t> *aoi;
  const Struct<int32_t> *asi;
  const Typedef<int32_t> *ati;
  Option<int32_t> *const *noi;
  Struct<int32_t> *const *nsi;
  Typedef<int32_t> *const *nti;
  Typedef<int32_t> **(*fi)(const Option<int32_t> *aoi,
                           const Struct<int32_t> *asi,
                           const Typedef<int32_t> *ati,
                           Option<int32_t> *const *noi,
                           Struct<int32_t> *const *nsi);
};

template<typename E = Option<Struct<int64_t>>>
struct NotTransparent2 {
  const Option<Option<E>> *oo;
  const Option<Struct<E>> *os;
  const Struct<Option<E>> *so;
  const Struct<Struct<E>> *ss;
  Struct<Struct<E>> *(*f)(const Option<Option<E>> *oo,
                          const Option<Struct<E>> *os,
                          const Struct<Option<E>> *so);
  const Option<Option<int32_t>> *ooi;
  const Option<Struct<int32_t>> *osi;
  const Struct<Option<int32_t>> *soi;
  const Struct<Struct<int32_t>> *ssi;
  Struct<Struct<int32_t>> *(*fi)(const Option<Option<int32_t>> *ooi,
                                 const Option<Struct<int32_t>> *osi,
                                 const Struct<Option<int32_t>> *soi);
};

template<typename E = int64_t*>
struct FullyTransparentMany {
  enum class Tag {
    ont,
    otn,
    ton,
    totn,
    f,
    onti,
    otni,
    toni,
    totni,
    fi,
  };

  struct ont_Body {
    E *_0;
  };

  struct otn_Body {
    E *_0;
  };

  struct ton_Body {
    E *_0;
  };

  struct totn_Body {
    E *_0;
  };

  struct f_Body {
    E *(*_0)(E *ont, E *otn, E *ton);
  };

  struct onti_Body {
    int32_t *_0;
  };

  struct otni_Body {
    int32_t *_0;
  };

  struct toni_Body {
    int32_t *_0;
  };

  struct totni_Body {
    int32_t *_0;
  };

  struct fi_Body {
    int32_t *(*_0)(int32_t *onti, int32_t *otni, int32_t *toni);
  };

  Tag tag;
  union {
    ont_Body ont;
    otn_Body otn;
    ton_Body ton;
    totn_Body totn;
    f_Body f;
    onti_Body onti;
    otni_Body otni;
    toni_Body toni;
    totni_Body totni;
    fi_Body fi;
  };
};

template<typename E = Option<int64_t>>
union PartlyTransparentMany {
  const Option<E> *tao;
  const Option<E> *toa;
  const Option<E> *ota;
  const Struct<E> *tas;
  const Struct<E> *tsa;
  const Struct<E> *sta;
  const Option<E> *toat;
  const Struct<E> *tsat;
  const Option<int32_t> *taoi;
  const Option<int32_t> *toai;
  const Option<int32_t> *otai;
  const Struct<int32_t> *tasi;
  const Struct<int32_t> *tsai;
  const Struct<int32_t> *stai;
  const Option<int32_t> *toati;
  const Struct<int32_t> *tsati;
};

extern "C" {

void root_opaque(const Opaque<int32_t> *o);

void root1(FullyTransparent1<int32_t> a, NotTransparent1<Option<int32_t>> s);

void root2(FullyTransparent2<int32_t> a,
           PartlyTransparent2<Option<int32_t>> s,
           NotTransparent2<Struct<Option<int32_t>>> n);

void root_many(FullyTransparentMany<int32_t*> a, PartlyTransparentMany<Option<int32_t>> b);

}  // extern "C"

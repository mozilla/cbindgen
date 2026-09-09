from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct Opaque_i32:
    pass

  cdef struct Option_Option_Struct_Option_i32:
    pass

  cdef struct Option_Option_i32:
    pass

  cdef struct Option_Struct_Option_i32:
    pass

  cdef struct Option_Struct_Struct_Option_i32:
    pass

  cdef struct Option_Struct_i32:
    pass

  cdef struct Option_i32:
    pass

  cdef struct FullyTransparent1_i32:
    int32_t a;
    int32_t *n;
    int32_t t;
    int32_t (*f)(int32_t a, int32_t *n);
    int32_t ai;
    int32_t *ni;
    int32_t ti;
    int32_t (*fi)(int32_t ai, int32_t *ni);

  cdef struct Struct_Option_i32:
    const Option_i32 *field;

  ctypedef Option_i32 Typedef_Option_i32;

  cdef struct Struct_i32:
    const int32_t *field;

  ctypedef int32_t Typedef_i32;

  cdef struct NotTransparent1_Option_i32:
    const Option_Option_i32 *o;
    const Struct_Option_i32 *s;
    const Typedef_Option_i32 *t;
    Typedef_Option_i32 *(*f)(const Option_Option_i32 *o, const Struct_Option_i32 *s);
    const Option_i32 *oi;
    const Struct_i32 *si;
    const Typedef_i32 *ti;
    Typedef_i32 *(*fi)(const Option_i32 *oi, const Struct_i32 *si);

  cdef struct FullyTransparent2_i32:
    int32_t aa;
    int32_t *an;
    int32_t at;
    int32_t *na;
    int32_t **nn;
    int32_t *nt;
    int32_t *on;
    int32_t ta;
    int32_t *tn;
    int32_t tt;
    int32_t (*f)(int32_t aa,
                 int32_t *an,
                 int32_t at,
                 int32_t *na,
                 int32_t **nn,
                 int32_t *nt,
                 int32_t *on,
                 int32_t ta,
                 int32_t *tn);
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

  cdef struct PartlyTransparent2_Option_i32:
    const Option_Option_i32 *ao;
    const Struct_Option_i32 *aS;
    const Typedef_Option_i32 *at;
    Option_Option_i32 *const *no;
    Struct_Option_i32 *const *ns;
    Typedef_Option_i32 *const *nt;
    Typedef_Option_i32 **(*f)(const Option_Option_i32 *ao,
                              const Struct_Option_i32 *aS,
                              const Typedef_Option_i32 *at,
                              Option_Option_i32 *const *no,
                              Struct_Option_i32 *const *ns);
    const Option_i32 *aoi;
    const Struct_i32 *asi;
    const Typedef_i32 *ati;
    Option_i32 *const *noi;
    Struct_i32 *const *nsi;
    Typedef_i32 *const *nti;
    Typedef_i32 **(*fi)(const Option_i32 *aoi,
                        const Struct_i32 *asi,
                        const Typedef_i32 *ati,
                        Option_i32 *const *noi,
                        Struct_i32 *const *nsi);

  cdef struct Struct_Option_Struct_Option_i32:
    const Option_Struct_Option_i32 *field;

  cdef struct Struct_Struct_Option_i32:
    const Struct_Option_i32 *field;

  cdef struct Struct_Struct_Struct_Option_i32:
    const Struct_Struct_Option_i32 *field;

  cdef struct Struct_Struct_i32:
    const Struct_i32 *field;

  cdef struct NotTransparent2_Struct_Option_i32:
    const Option_Option_Struct_Option_i32 *oo;
    const Option_Struct_Struct_Option_i32 *os;
    const Struct_Option_Struct_Option_i32 *so;
    const Struct_Struct_Struct_Option_i32 *ss;
    Struct_Struct_Struct_Option_i32 *(*f)(const Option_Option_Struct_Option_i32 *oo,
                                          const Option_Struct_Struct_Option_i32 *os,
                                          const Struct_Option_Struct_Option_i32 *so);
    const Option_Option_i32 *ooi;
    const Option_Struct_i32 *osi;
    const Struct_Option_i32 *soi;
    const Struct_Struct_i32 *ssi;
    Struct_Struct_i32 *(*fi)(const Option_Option_i32 *ooi,
                             const Option_Struct_i32 *osi,
                             const Struct_Option_i32 *soi);

  cdef enum FullyTransparentMany_____i32_Tag:
    ont_____i32,
    otn_____i32,
    ton_____i32,
    totn_____i32,
    f_____i32,
    onti_____i32,
    otni_____i32,
    toni_____i32,
    totni_____i32,
    fi_____i32,

  cdef struct FullyTransparentMany_____i32:
    FullyTransparentMany_____i32_Tag tag;
    int32_t **ont;
    int32_t **otn;
    int32_t **ton;
    int32_t **totn;
    int32_t **(*f)(int32_t **ont, int32_t **otn, int32_t **ton);
    int32_t *onti;
    int32_t *otni;
    int32_t *toni;
    int32_t *totni;
    int32_t *(*fi)(int32_t *onti, int32_t *otni, int32_t *toni);

  cdef union PartlyTransparentMany_Option_i32:
    const Option_Option_i32 *tao;
    const Option_Option_i32 *toa;
    const Option_Option_i32 *ota;
    const Struct_Option_i32 *tas;
    const Struct_Option_i32 *tsa;
    const Struct_Option_i32 *sta;
    const Option_Option_i32 *toat;
    const Struct_Option_i32 *tsat;
    const Option_i32 *taoi;
    const Option_i32 *toai;
    const Option_i32 *otai;
    const Struct_i32 *tasi;
    const Struct_i32 *tsai;
    const Struct_i32 *stai;
    const Option_i32 *toati;
    const Struct_i32 *tsati;

  void root_opaque(const Opaque_i32 *o);

  void root1(FullyTransparent1_i32 a, NotTransparent1_Option_i32 s);

  void root2(FullyTransparent2_i32 a,
             PartlyTransparent2_Option_i32 s,
             NotTransparent2_Struct_Option_i32 n);

  void root_many(FullyTransparentMany_____i32 a, PartlyTransparentMany_Option_i32 b);

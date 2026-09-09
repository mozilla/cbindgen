#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque_i32 Opaque_i32;

typedef struct Option_Option_Struct_Option_i32 Option_Option_Struct_Option_i32;

typedef struct Option_Option_i32 Option_Option_i32;

typedef struct Option_Struct_Option_i32 Option_Struct_Option_i32;

typedef struct Option_Struct_Struct_Option_i32 Option_Struct_Struct_Option_i32;

typedef struct Option_Struct_i32 Option_Struct_i32;

typedef struct Option_i32 Option_i32;

typedef struct FullyTransparent1_i32 {
  int32_t a;
  int32_t *n;
  int32_t t;
  int32_t (*f)(int32_t a, int32_t *n);
  int32_t ai;
  int32_t *ni;
  int32_t ti;
  int32_t (*fi)(int32_t ai, int32_t *ni);
} FullyTransparent1_i32;

typedef struct Struct_Option_i32 {
  const struct Option_i32 *field;
} Struct_Option_i32;

typedef struct Option_i32 Typedef_Option_i32;

typedef struct Struct_i32 {
  const int32_t *field;
} Struct_i32;

typedef int32_t Typedef_i32;

typedef struct NotTransparent1_Option_i32 {
  const struct Option_Option_i32 *o;
  const struct Struct_Option_i32 *s;
  const Typedef_Option_i32 *t;
  Typedef_Option_i32 *(*f)(const struct Option_Option_i32 *o, const struct Struct_Option_i32 *s);
  const struct Option_i32 *oi;
  const struct Struct_i32 *si;
  const Typedef_i32 *ti;
  Typedef_i32 *(*fi)(const struct Option_i32 *oi, const struct Struct_i32 *si);
} NotTransparent1_Option_i32;

typedef struct FullyTransparent2_i32 {
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
} FullyTransparent2_i32;

typedef struct PartlyTransparent2_Option_i32 {
  const struct Option_Option_i32 *ao;
  const struct Struct_Option_i32 *aS;
  const Typedef_Option_i32 *at;
  struct Option_Option_i32 *const *no;
  struct Struct_Option_i32 *const *ns;
  Typedef_Option_i32 *const *nt;
  Typedef_Option_i32 **(*f)(const struct Option_Option_i32 *ao,
                            const struct Struct_Option_i32 *aS,
                            const Typedef_Option_i32 *at,
                            struct Option_Option_i32 *const *no,
                            struct Struct_Option_i32 *const *ns);
  const struct Option_i32 *aoi;
  const struct Struct_i32 *asi;
  const Typedef_i32 *ati;
  struct Option_i32 *const *noi;
  struct Struct_i32 *const *nsi;
  Typedef_i32 *const *nti;
  Typedef_i32 **(*fi)(const struct Option_i32 *aoi,
                      const struct Struct_i32 *asi,
                      const Typedef_i32 *ati,
                      struct Option_i32 *const *noi,
                      struct Struct_i32 *const *nsi);
} PartlyTransparent2_Option_i32;

typedef struct Struct_Option_Struct_Option_i32 {
  const struct Option_Struct_Option_i32 *field;
} Struct_Option_Struct_Option_i32;

typedef struct Struct_Struct_Option_i32 {
  const struct Struct_Option_i32 *field;
} Struct_Struct_Option_i32;

typedef struct Struct_Struct_Struct_Option_i32 {
  const struct Struct_Struct_Option_i32 *field;
} Struct_Struct_Struct_Option_i32;

typedef struct Struct_Struct_i32 {
  const struct Struct_i32 *field;
} Struct_Struct_i32;

typedef struct NotTransparent2_Struct_Option_i32 {
  const struct Option_Option_Struct_Option_i32 *oo;
  const struct Option_Struct_Struct_Option_i32 *os;
  const struct Struct_Option_Struct_Option_i32 *so;
  const struct Struct_Struct_Struct_Option_i32 *ss;
  struct Struct_Struct_Struct_Option_i32 *(*f)(const struct Option_Option_Struct_Option_i32 *oo,
                                               const struct Option_Struct_Struct_Option_i32 *os,
                                               const struct Struct_Option_Struct_Option_i32 *so);
  const struct Option_Option_i32 *ooi;
  const struct Option_Struct_i32 *osi;
  const struct Struct_Option_i32 *soi;
  const struct Struct_Struct_i32 *ssi;
  struct Struct_Struct_i32 *(*fi)(const struct Option_Option_i32 *ooi,
                                  const struct Option_Struct_i32 *osi,
                                  const struct Struct_Option_i32 *soi);
} NotTransparent2_Struct_Option_i32;

typedef enum FullyTransparentMany_____i32_Tag {
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
} FullyTransparentMany_____i32_Tag;

typedef struct FullyTransparentMany_____i32 {
  FullyTransparentMany_____i32_Tag tag;
  union {
    struct {
      int32_t **ont;
    };
    struct {
      int32_t **otn;
    };
    struct {
      int32_t **ton;
    };
    struct {
      int32_t **totn;
    };
    struct {
      int32_t **(*f)(int32_t **ont, int32_t **otn, int32_t **ton);
    };
    struct {
      int32_t *onti;
    };
    struct {
      int32_t *otni;
    };
    struct {
      int32_t *toni;
    };
    struct {
      int32_t *totni;
    };
    struct {
      int32_t *(*fi)(int32_t *onti, int32_t *otni, int32_t *toni);
    };
  };
} FullyTransparentMany_____i32;

typedef union PartlyTransparentMany_Option_i32 {
  const struct Option_Option_i32 *tao;
  const struct Option_Option_i32 *toa;
  const struct Option_Option_i32 *ota;
  const struct Struct_Option_i32 *tas;
  const struct Struct_Option_i32 *tsa;
  const struct Struct_Option_i32 *sta;
  const struct Option_Option_i32 *toat;
  const struct Struct_Option_i32 *tsat;
  const struct Option_i32 *taoi;
  const struct Option_i32 *toai;
  const struct Option_i32 *otai;
  const struct Struct_i32 *tasi;
  const struct Struct_i32 *tsai;
  const struct Struct_i32 *stai;
  const struct Option_i32 *toati;
  const struct Struct_i32 *tsati;
} PartlyTransparentMany_Option_i32;

void root_opaque(const struct Opaque_i32 *o);

void root1(struct FullyTransparent1_i32 a, struct NotTransparent1_Option_i32 s);

void root2(struct FullyTransparent2_i32 a,
           struct PartlyTransparent2_Option_i32 s,
           struct NotTransparent2_Struct_Option_i32 n);

void root_many(struct FullyTransparentMany_____i32 a, union PartlyTransparentMany_Option_i32 b);

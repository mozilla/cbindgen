#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  int32_t namespace_;
  float float_;
} A;

typedef struct {
  int32_t namespace_;
  float float_;
} B;

enum C_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  D,
};
#if __STDC_VERSION__ >= 202311L
typedef enum C_Tag C_Tag;
#else
typedef uint8_t C_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  int32_t namespace_;
  float float_;
} D_Body;

typedef struct {
  C_Tag tag;
  union {
    D_Body d;
  };
} C;

enum E_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  Double,
  Float,
};
#if __STDC_VERSION__ >= 202311L
typedef enum E_Tag E_Tag;
#else
typedef uint8_t E_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  E_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
} E;

enum F_Tag
#if __STDC_VERSION__ >= 202311L
  : uint8_t
#endif // __STDC_VERSION__ >= 202311L
 {
  double_,
  float_,
};
#if __STDC_VERSION__ >= 202311L
typedef enum F_Tag F_Tag;
#else
typedef uint8_t F_Tag;
#endif // __STDC_VERSION__ >= 202311L

typedef struct {
  F_Tag tag;
  union {
    struct {
      double double_;
    };
    struct {
      float float_;
    };
  };
} F;

void root(A a, B b, C c, E e, F f, int32_t namespace_, float float_);

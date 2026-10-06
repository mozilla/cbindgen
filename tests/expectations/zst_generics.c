#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint32_t other_field;
} StructWithGenArray_1Z;

typedef union {
  uint32_t b;
} U_1Z;

typedef enum {
  A_1Z,
  B_1Z,
  C_1Z,
} E_1Z_Tag;

typedef struct {
  uint32_t y;
} A_Body_1Z;

typedef struct {
  E_1Z_Tag tag;
  union {
    A_Body_1Z a;
    struct {

    };
  };
} E_1Z;

typedef struct {
  uint32_t b;
} CG_1Z__3;

typedef struct {
  uint32_t k;
} Two_1Z__1Z;

typedef struct {
  uint8_t r;
  uint32_t k;
} Two_1Z__u8;

typedef struct {
  uint8_t t;
  uint32_t k;
} Two_u8__1Z;

typedef struct {
  uint32_t b;
} Wrap1_1Z;

typedef struct {
  Wrap1_1Z w;
  uint16_t c;
} Wrap2_1Z;

typedef struct {
  Wrap1_1Z a;
  uint32_t b;
} Wrap1_Wrap1_1Z;

typedef struct {
  uint8_t t;
  uint32_t k;
} D2_u8;

typedef struct {
  uint8_t t;
  uint32_t k;
} D2_u8__1Z;

void use_struct(StructWithGenArray_1Z arg);

void use_union(U_1Z x);

void use_enum(E_1Z x);

void use_cg(CG_1Z__3 x);

void use_two(Two_1Z__1Z x, Two_1Z__u8 y, Two_u8__1Z z);

void use_nested(Wrap2_1Z x, Wrap1_Wrap1_1Z y);

void use_d2(D2_u8 x, D2_u8__1Z y);

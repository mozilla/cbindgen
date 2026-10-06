#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct StructWithGenArray_1Z {
  uint32_t other_field;
} StructWithGenArray_1Z;

typedef union U_1Z {
  uint32_t b;
} U_1Z;

typedef enum E_1Z_Tag {
  A_1Z,
  B_1Z,
  C_1Z,
} E_1Z_Tag;

typedef struct A_Body_1Z {
  uint32_t y;
} A_Body_1Z;

typedef struct E_1Z {
  E_1Z_Tag tag;
  union {
    A_Body_1Z a;
    struct {

    };
  };
} E_1Z;

typedef struct CG_1Z__3 {
  uint32_t b;
} CG_1Z__3;

typedef struct Two_1Z__1Z {
  uint32_t k;
} Two_1Z__1Z;

typedef struct Two_1Z__u8 {
  uint8_t r;
  uint32_t k;
} Two_1Z__u8;

typedef struct Two_u8__1Z {
  uint8_t t;
  uint32_t k;
} Two_u8__1Z;

typedef struct Wrap1_1Z {
  uint32_t b;
} Wrap1_1Z;

typedef struct Wrap2_1Z {
  struct Wrap1_1Z w;
  uint16_t c;
} Wrap2_1Z;

typedef struct Wrap1_Wrap1_1Z {
  struct Wrap1_1Z a;
  uint32_t b;
} Wrap1_Wrap1_1Z;

typedef struct D2_u8 {
  uint8_t t;
  uint32_t k;
} D2_u8;

typedef struct D2_u8__1Z {
  uint8_t t;
  uint32_t k;
} D2_u8__1Z;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_struct(struct StructWithGenArray_1Z arg);

void use_union(union U_1Z x);

void use_enum(struct E_1Z x);

void use_cg(struct CG_1Z__3 x);

void use_two(struct Two_1Z__1Z x, struct Two_1Z__u8 y, struct Two_u8__1Z z);

void use_nested(struct Wrap2_1Z x, struct Wrap1_Wrap1_1Z y);

void use_d2(struct D2_u8 x, struct D2_u8__1Z y);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

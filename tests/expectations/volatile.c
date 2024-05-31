#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef volatile int V_c_int;

typedef V_c_int Vint;

typedef volatile Vint V_Vint;

typedef V_Vint Vvint;

typedef int *volatile V_____c_int;

typedef V_____c_int Vpint;

typedef const int *volatile V______c_int;

typedef V______c_int Vpcint;

typedef volatile uint32_t V_u32;

typedef V_u32 Vnzu32;

typedef V_____c_int Vnnint;

typedef V_c_int Vcint;

typedef void (*volatile V_______c_void)(void);

typedef V_______c_void Vfn;

typedef struct {
  volatile int vfield;
  V_c_int vint;
  V_Vint vvint;
  V_____c_int vpint;
  V______c_int vpcint;
  V_u32 vnzu32;
  V_____c_int vnnint;
  V_c_int vcint;
  V_______c_void vfn;
  V_c_int a1vint[1];
} S;

typedef union {
  volatile int vfield;
  V_c_int vint;
  V_Vint vvint;
  V_____c_int vpint;
  V______c_int vpcint;
  V_u32 vnzu32;
  V_____c_int vnnint;
  V_______c_void vfn;
  V_c_int a1vint[1];
} U;

extern V_c_int g_vint;

extern V_Vint g_vvint;

extern V_____c_int g_vpint;

extern V______c_int g_vpcint;

extern V_u32 g_vnzu32;

extern V_____c_int g_vnnint;

extern V_c_int g_vcint;

extern V_______c_void g_vfn;

extern V_c_int g_a1vint[1];

void _export(Vint, Vvint, Vpint, Vpcint, Vnzu32, Vnnint, Vcint, Vfn, S, U);

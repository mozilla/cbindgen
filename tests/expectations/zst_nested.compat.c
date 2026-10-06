#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Opaque_1Z Opaque_1Z;

typedef struct Opaque_______c_void Opaque_______c_void;

typedef enum {
  Some_______c_void,
  None_______c_void,
} MyOption_______c_void_Tag;

typedef struct {
  MyOption_______c_void_Tag tag;
  union {
    struct {
      void (*some)(void);
    };
  };
} MyOption_______c_void;

typedef struct {
  void (*inner)(void);
} MyWrapper_______c_void;

typedef struct {
  void (*args_erased)(void);
  void (*args_erased_with_npo)(void);
  MyOption_______c_void args_erased_with_custom_enum;
  MyWrapper_______c_void args_erased_with_custom_struct;
  const Opaque_______c_void *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(void);
  void (*args_erased_in_array[2])(void);
  void (*(*args_erased_in_ret)(void))(void);
  const void *arr_erased_behind_ptr;
  const Opaque_1Z *arr_erased_in_opaque;
} S_1Z;

typedef struct {
  void (*f)(uint8_t[4]);
  uint8_t x[4];
} S2_1Z___________u8__________4;

typedef struct {
  const void *p;
} Inner_1Z;

typedef struct {
  Inner_1Z plain;
  Inner_1Z arr;
} Outer_1Z;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_S(S_1Z arg1, S2_1Z___________u8__________4 arg2, Outer_1Z arg3);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct Opaque_1Z;

struct Opaque_______c_void;

enum MyOption_______c_void_Tag {
  Some_______c_void,
  None_______c_void,
};

struct MyOption_______c_void {
  enum MyOption_______c_void_Tag tag;
  union {
    struct {
      void (*some)(void);
    };
  };
};

struct MyWrapper_______c_void {
  void (*inner)(void);
};

struct S_1Z {
  void (*args_erased)(void);
  void (*args_erased_with_npo)(void);
  struct MyOption_______c_void args_erased_with_custom_enum;
  struct MyWrapper_______c_void args_erased_with_custom_struct;
  const struct Opaque_______c_void *args_erased_with_opaque;
  void (**args_erased_behind_ptr)(void);
  void (*args_erased_in_array[2])(void);
  void (*(*args_erased_in_ret)(void))(void);
  const void *arr_erased_behind_ptr;
  const struct Opaque_1Z *arr_erased_in_opaque;
};

struct S2_1Z___________u8__________4 {
  void (*f)(uint8_t[4]);
  uint8_t x[4];
};

struct Inner_1Z {
  const void *p;
};

struct Outer_1Z {
  struct Inner_1Z plain;
  struct Inner_1Z arr;
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_S(struct S_1Z arg1, struct S2_1Z___________u8__________4 arg2, struct Outer_1Z arg3);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

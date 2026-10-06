#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Opaque;

template<typename T = void>
struct Option;

template<typename T = void, typename E = void>
struct Result;

template<typename T, uintptr_t N>
struct CG {
  T a[N];
  uint32_t b;
};

template<typename T>
struct ValidPositions {
  void (*args_always_erased)();
  const T *zst_behind_ptr;
  const Opaque<T> *zst_in_opaque;
  T (*f_returns_zst)(uint32_t arg);
};

struct APIError {
  uint32_t err;
};

template<typename O, typename E>
struct CResultTempl {
  bool result_good;
  const O *result;
  const E *err;
};

using CResultNoneAPIError = CResultTempl<void, APIError>;

template<typename T>
struct Foo {
  uint32_t x;
};

extern "C" {

void option_u32(Option<uint32_t> arg);

void option_unit(Option<void> arg, Option<Option<void>> nested);

void result_zsts(Result<void, uint32_t> arg1, Result<uint32_t, void> arg2, Result<void, void> arg3);

void use_cg(CG<uint8_t, 3> x);

void use_valid_positions(ValidPositions<void> arg);

void CResultNoneAPIError_free(CResultNoneAPIError _res);

Foo<void> bar();

}  // extern "C"

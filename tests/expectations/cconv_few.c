#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


// Compiler-specific cdecl calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#cdecl
#define __cbindgen_abi_cdecl __cdecl
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_cdecl __cdecl
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-cdecl-function-attribute_002c-x86-32
#define __cbindgen_abi_cdecl __attribute__((cdecl))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/cdecl?view=msvc-170
#define __cbindgen_abi_cdecl __cdecl
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"cdecl\" may break at runtime." )
#define __cbindgen_abi_cdecl
#endif

// Compiler-specific stdcall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#stdcall
#define __cbindgen_abi_stdcall __stdcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_stdcall __stdcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-stdcall-function-attribute_002c-x86-32
#define __cbindgen_abi_stdcall __attribute__((stdcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/stdcall?view=msvc-170
#define __cbindgen_abi_stdcall __stdcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"stdcall\" may break at runtime." )
#define __cbindgen_abi_stdcall
#endif

void test_c(void);

void __cbindgen_abi_cdecl test_cdecl(void);

void __cbindgen_abi_stdcall test_stdcall(void);

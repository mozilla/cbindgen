#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// Compiler-specific thiscall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#thiscall
#define __cbindgen_abi_thiscall_unwind __thiscall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_thiscall_unwind __thiscall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-thiscall-function-attribute_002c-x86-32
#define __cbindgen_abi_thiscall_unwind __attribute__((thiscall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/thiscall?view=msvc-170
#define __cbindgen_abi_thiscall_unwind __thiscall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"thiscall\" may break at runtime." )
#define __cbindgen_abi_thiscall_unwind
#endif

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

// Compiler-specific efiapi calling convention definition
#if (defined(__arm__) && !defined(__aarch64__)) || defined(_M_ARM)
// On ARM, EFIAPI is the same as AAPCS
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#pcs
#define __cbindgen_abi_efiapi __attribute__((pcs("aapcs")))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_efiapi __attribute__((pcs("aapcs")))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/ARM-Function-Attributes.html#index-pcs-function-attribute_002c-ARM
#define __cbindgen_abi_efiapi __attribute__((pcs("aapcs")))
#elif defined(_MSC_VER)
// MSVC: Does not support an attribute for AAPCS, but it is the default
// as described in: https://learn.microsoft.com/en-us/cpp/build/overview-of-arm-abi-conventions?view=msvc-170
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"efiapi\" may break at runtime." )
#define __cbindgen_abi_efiapi
#endif
#elif defined(__x86_64__) || defined(_M_X64)
// On x86_64, EFIAPI is MS_ABI
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#ms-abi
#define __cbindgen_abi_efiapi __attribute__((ms_abi))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_efiapi __attribute__((ms_abi))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-ms_005fabi-function-attribute_002c-x86
#define __cbindgen_abi_efiapi __attribute__((ms_abi))
#elif defined(_MSC_VER)
// MSVC: ms_abi is the default ABI on MSVC and does not need to be specified
#define __cbindgen_abi_efiapi
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"efiapi\" may break at runtime." )
#define __cbindgen_abi_efiapi
#endif
#else
// On all other architectures, EFIAPI is a no-op
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
#define __cbindgen_abi_efiapi
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
#define __cbindgen_abi_efiapi
#elif defined(__GNUC__) || defined(__GNUG__)
#define __cbindgen_abi_efiapi
#elif defined(_MSC_VER)
#define __cbindgen_abi_efiapi
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"efiapi\" may break at runtime." )
#define __cbindgen_abi_efiapi
#endif
#endif

// Compiler-specific aapcs calling convention definition
#if defined(__arm__) || defined(_M_ARM)
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#pcs
#define __cbindgen_abi_aapcs_unwind __attribute__((pcs("aapcs")))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_aapcs_unwind __attribute__((pcs("aapcs")))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/ARM-Function-Attributes.html#index-pcs-function-attribute_002c-ARM
#define __cbindgen_abi_aapcs_unwind __attribute__((pcs("aapcs")))
#elif defined(_MSC_VER)
// MSVC: Does not support an attribute for AAPCS, but it is the default
// as described in: https://learn.microsoft.com/en-us/cpp/build/overview-of-arm-abi-conventions?view=msvc-170
#define __cbindgen_abi_aapcs_unwind
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"aapcs\" may break at runtime." )
#define __cbindgen_abi_aapcs_unwind
#endif
#else
#pragma message ( "The AAPCS ABI is not available on non-ARM platforms but has been requested. This may result in code which breaks at runtime." )
#define __cbindgen_abi_aapcs_unwind
#endif

// Compiler-specific fastcall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#fastcall
#define __cbindgen_abi_fastcall_unwind __fastcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_fastcall_unwind __fastcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-fastcall-function-attribute_002c-x86-32
#define __cbindgen_abi_fastcall_unwind __attribute__((fastcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/fastcall?view=msvc-170
#define __cbindgen_abi_fastcall_unwind __fastcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"fastcall\" may break at runtime." )
#define __cbindgen_abi_fastcall_unwind
#endif

// Compiler-specific system calling convention definition
#if (defined(_WIN32) || defined(__WIN32__) || defined(__WIN32)) && (defined(__i386__) || defined(_M_IX86))
// If we are targeting 32-bit windows, "system" is "stdcall"
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#system
#define __cbindgen_abi_system __stdcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_system __stdcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-system-function-attribute_002c-x86-32
#define __cbindgen_abi_system __attribute__((stdcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/system?view=msvc-170
#define __cbindgen_abi_system __stdcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"system\" may break at runtime." )
#define __cbindgen_abi_system
#endif
#else
// Otherwise, it is equivalent to "C" AKA empty
#define __cbindgen_abi_system
#endif

// Compiler-specific fastcall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#fastcall
#define __cbindgen_abi_fastcall __fastcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_fastcall __fastcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-fastcall-function-attribute_002c-x86-32
#define __cbindgen_abi_fastcall __attribute__((fastcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/fastcall?view=msvc-170
#define __cbindgen_abi_fastcall __fastcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"fastcall\" may break at runtime." )
#define __cbindgen_abi_fastcall
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

// Compiler-specific win64 calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#ms-abi
#define __cbindgen_abi_win64 __attribute__((ms_abi))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_win64 __attribute__((ms_abi))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-ms_005fabi-function-attribute_002c-x86
#define __cbindgen_abi_win64 __attribute__((ms_abi))
#elif defined(_MSC_VER)
// MSVC: ms_abi is the default ABI on MSVC and does not need to be specified
#define __cbindgen_abi_win64
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"win64\" may break at runtime." )
#define __cbindgen_abi_win64
#endif

// Compiler-specific stdcall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#stdcall
#define __cbindgen_abi_stdcall_unwind __stdcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_stdcall_unwind __stdcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-stdcall-function-attribute_002c-x86-32
#define __cbindgen_abi_stdcall_unwind __attribute__((stdcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/stdcall?view=msvc-170
#define __cbindgen_abi_stdcall_unwind __stdcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"stdcall\" may break at runtime." )
#define __cbindgen_abi_stdcall_unwind
#endif

// Compiler-specific thiscall calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#thiscall
#define __cbindgen_abi_thiscall __thiscall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_thiscall __thiscall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-thiscall-function-attribute_002c-x86-32
#define __cbindgen_abi_thiscall __attribute__((thiscall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/thiscall?view=msvc-170
#define __cbindgen_abi_thiscall __thiscall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"thiscall\" may break at runtime." )
#define __cbindgen_abi_thiscall
#endif

// Compiler-specific system calling convention definition
#if (defined(_WIN32) || defined(__WIN32__) || defined(__WIN32)) && (defined(__i386__) || defined(_M_IX86))
// If we are targeting 32-bit windows, "system" is "stdcall"
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#system
#define __cbindgen_abi_system_unwind __stdcall
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_system_unwind __stdcall
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-system-function-attribute_002c-x86-32
#define __cbindgen_abi_system_unwind __attribute__((stdcall))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/system?view=msvc-170
#define __cbindgen_abi_system_unwind __stdcall
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"system\" may break at runtime." )
#define __cbindgen_abi_system_unwind
#endif
#else
// Otherwise, it is equivalent to "C" AKA empty
#define __cbindgen_abi_system_unwind
#endif

// Compiler-specific cdecl calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#cdecl
#define __cbindgen_abi_cdecl_unwind __cdecl
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_cdecl_unwind __cdecl
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-cdecl-function-attribute_002c-x86-32
#define __cbindgen_abi_cdecl_unwind __attribute__((cdecl))
#elif defined(_MSC_VER)
// MSVC: https://learn.microsoft.com/en-us/cpp/cpp/cdecl?view=msvc-170
#define __cbindgen_abi_cdecl_unwind __cdecl
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"cdecl\" may break at runtime." )
#define __cbindgen_abi_cdec_unwindl
#endif

// Compiler-specific aapcs calling convention definition
#if defined(__arm__) || defined(_M_ARM)
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#pcs
#define __cbindgen_abi_aapcs __attribute__((pcs("aapcs")))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_aapcs __attribute__((pcs("aapcs")))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/ARM-Function-Attributes.html#index-pcs-function-attribute_002c-ARM
#define __cbindgen_abi_aapcs __attribute__((pcs("aapcs")))
#elif defined(_MSC_VER)
// MSVC: Does not support an attribute for AAPCS, but it is the default
// as described in: https://learn.microsoft.com/en-us/cpp/build/overview-of-arm-abi-conventions?view=msvc-170
#define __cbindgen_abi_aapcs
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"aapcs\" may break at runtime." )
#define __cbindgen_abi_aapcs
#endif
#else
#pragma message ( "The AAPCS ABI is not available on non-ARM platforms but has been requested. This may result in code which breaks at runtime." )
#define __cbindgen_abi_aapcs
#endif

// Compiler-specific win64 calling convention definition
#if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
// Clang: https://clang.llvm.org/docs/AttributeReference.html#ms-abi
#define __cbindgen_abi_win64_unwind __attribute__((ms_abi))
#elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
// ICX: See Clang
#define __cbindgen_abi_win64_unwind __attribute__((ms_abi))
#elif defined(__GNUC__) || defined(__GNUG__)
// GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-ms_005fabi-function-attribute_002c-x86
#define __cbindgen_abi_win64_unwind __attribute__((ms_abi))
#elif defined(_MSC_VER)
// MSVC: ms_abi is the default ABI on MSVC and does not need to be specified
#define __cbindgen_abi_win64_unwind
#else
#pragma message ( "An unsupported compiler is in use. Functions declared as extern \"win64\" may break at runtime." )
#define __cbindgen_abi_win64
#endif


void test_none(void);

void test_c(void);

void __cbindgen_abi_cdecl test_cdecl(void);

void __cbindgen_abi_stdcall test_stdcall(void);

void __cbindgen_abi_win64 test_win64(void);

void test_sysv64(void);

void __cbindgen_abi_system test_rust(void);

void __cbindgen_abi_aapcs test_aapcs(void);

void __cbindgen_abi_fastcall test_fastcall(void);

void __cbindgen_abi_thiscall test_thiscall(void);

void __cbindgen_abi_efiapi test_efiapi(void);

void test_c(void);

void __cbindgen_abi_cdecl test_cdecl(void);

void __cbindgen_abi_stdcall test_stdcall(void);

void __cbindgen_abi_win64 test_win64(void);

void test_sysv64(void);

void __cbindgen_abi_system test_rust(void);

void __cbindgen_abi_aapcs test_aapcs(void);

void __cbindgen_abi_fastcall test_fastcall(void);

void __cbindgen_abi_thiscall test_thiscall(void);

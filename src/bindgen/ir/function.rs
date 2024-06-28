/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use syn::ext::IdentExt;

use indoc::indoc;

use crate::bindgen::config::{Config, Language};
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{AnnotationSet, Cfg, Documentation, GenericPath, Path, Type};
use crate::bindgen::library::Library;
use crate::bindgen::monomorph::Monomorphs;
use crate::bindgen::rename::{IdentifierType, RenameRule};
use crate::bindgen::reserved;
use crate::bindgen::utilities::IterHelpers;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A function ABI in Rust. Rust ABIs are defined in:
///
/// https://github.com/rust-lang/rust/blob/9c3bc805dd9cb84019c124b9a50fdff1e62a7ec9/compiler/rustc_target/src/spec/abi/mod.r
///
/// There are a few more ABIs, but only stable ABIs are supported currently.
///
/// See the compiler explorer for a test of all the ABIs:
/// https://godbolt.org/z/PsEPzGz1P
///
pub enum FunctionAbi {
    /// No specified ABI, same as extern "C"
    None,
    /// The default the C compiler supports
    C,
    /// CDecl, the default for x86_32 C code
    CDecl,
    /// The default for the Win32 API on x86_32
    StdCall,
    /// The default for C code on x86_64 windows
    Win64,
    /// The default for C code on non-windows x86_64
    SystemV64,
    /// Same as extern "C" except on Win32, where it is "stdcall"
    System,
    /// The default for the ARM architecture
    AApcs,
    /// MSVC's __fastcall/__attribute__((fastcall))
    FastCall,
    /// MSVC's __thiscall/__attribute__((thiscall))
    ThisCall,
    /// ABI used for UEFI
    EfiApi,
    /// Same as "C" but with unwinding
    CUnwind,
    /// Same as "cdecl" but with unwinding
    CDeclUnwind,
    /// Same as "stdcall" but with unwinding
    StdCallUnwind,
    /// Same as "win64" but with unwinding
    Win64Unwind,
    /// Same as "systemv64" but with unwinding
    SystemV64Unwind,
    /// Same as "system" but with unwinding
    SystemUnwind,
    /// Same as "aapcs" but with unwinding
    AApcsUnwind,
    /// Same as "fastcall" but with unwinding
    FastCallUnwind,
    /// Same as "thiscall" but with unwinding
    ThisCallUnwind,
}

impl FunctionAbi {
    pub(crate) fn none() -> Self {
        FunctionAbi::None
    }

    pub(crate) fn abi(abi: &syn::Abi) -> Self {
        abi.name
            .as_ref()
            .map_or_else(FunctionAbi::none, |name| match name.value().as_str() {
                "C" => FunctionAbi::C,
                "cdecl" => FunctionAbi::CDecl,
                "stdcall" => FunctionAbi::StdCall,
                "win64" => FunctionAbi::Win64,
                "systemv64" => FunctionAbi::SystemV64,
                "system" => FunctionAbi::System,
                "aapcs" => FunctionAbi::AApcs,
                "fastcall" => FunctionAbi::FastCall,
                "thiscall" => FunctionAbi::ThisCall,
                "efiapi" => FunctionAbi::EfiApi,
                "C-unwind" => FunctionAbi::CUnwind,
                "cdecl-unwind" => FunctionAbi::CDeclUnwind,
                "stdcall-unwind" => FunctionAbi::StdCallUnwind,
                "win64-unwind" => FunctionAbi::Win64Unwind,
                "systemv64-unwind" => FunctionAbi::SystemV64Unwind,
                "system-unwind" => FunctionAbi::SystemUnwind,
                "aapcs-unwind" => FunctionAbi::AApcsUnwind,
                "fastcall-unwind" => FunctionAbi::FastCallUnwind,
                "thiscall-unwind" => FunctionAbi::ThisCallUnwind,
                _ => FunctionAbi::none(),
            })
    }

    /// Convert to an attribute that can be applied to a function declaration in the MSVC
    /// format. We emit a define for the attribute in the generated code like, but only if the
    /// attribute is actually used.
    ///
    /// ```c
    /// #if defined(_MSC_VER)
    /// #define __cbindgen_cdecl __cdecl
    /// #else
    /// #define __cbindgen_cdecl __attribute__((cdecl))
    /// #endif
    /// ```
    ///
    /// `__cbindgen_` is used as a prefix to avoid conflicts with other attributes and
    /// to make it clear that this is a bindgen-specific attribute, as well as to avoid
    /// conflicts with possible redefinitions of the attribute.
    pub(crate) fn as_attribute(&self) -> Option<&'static str> {
        match *self {
            // Blank is equivalent to "C" and is the default for the compiler, so it is not explicitly specified
            // C is the default C abi for the compiler, so it is not expliclty specified
            FunctionAbi::None | FunctionAbi::C | FunctionAbi::CUnwind => None,
            // CDecl is a specific ABI, so it is specified.
            // CDecl is available as:
            // - __cdecl in MSVC
            // - __attribute__((cdecl)) in GCC/Clang
            FunctionAbi::CDecl => Some("__cbindgen_abi_cdecl"),
            // Stdcall is a specific ABI, so it is specified.
            // Stdcall is available as:
            // - __stdcall in MSVC
            // - __attribute__((stdcall)) in GCC/Clang
            FunctionAbi::StdCall => Some("__cbindgen_abi_stdcall"),
            // Win64 can't be specified specifically), but is used when ms_abi is
            // specified for 64-bit targets. It can't be used on 32-bit targets), so it
            // should be safe to emit it without further conditions when requested. It is the
            // default for MSVC), so it is explicitly defined as empty on that target.
            FunctionAbi::Win64 => Some("__cbindgen_abi_win64"),
            // Sysv is a specific ABI), so it is specified. It is not available on MSVC however),
            // so it is defined as empty on that target.
            FunctionAbi::SystemV64 => Some("__cbindgen_abi_sysv64"),
            // Means "C" on all but 32-bit windows targets), where it means "stdcall"
            FunctionAbi::System => Some("__cbindgen_abi_system"),
            // The default on aarch64), but not available as a specific attribute. That
            // means it should be safe to emit an empty attribute), and it will be
            // compiled as the default cconv on that platform.
            FunctionAbi::AApcs => Some("__cbindgen_abi_aapcs"),
            // Fastcall is a specific ABI), so it is specified.
            // Fastcall is available as:
            // - __fastcall in MSVC
            // - __attribute__((fastcall)) in GCC/Clang
            FunctionAbi::FastCall => Some("__cbindgen_abi_fastcall"),
            FunctionAbi::ThisCall => Some("__cbindgen_abi_thiscall"),
            FunctionAbi::EfiApi => Some("__cbindgen_abi_efiapi"),
            FunctionAbi::CDeclUnwind => Some("__cbindgen_abi_cdecl"),
            FunctionAbi::StdCallUnwind => Some("__cbindgen_abi_stdcall"),
            FunctionAbi::Win64Unwind => Some("__cbindgen_abi_win64"),
            FunctionAbi::SystemV64Unwind => Some("__cbindgen_abi_sysv64"),
            FunctionAbi::SystemUnwind => Some("__cbindgen_abi_system"),
            FunctionAbi::AApcsUnwind => Some("__cbindgen_abi_aapcs"),
            FunctionAbi::FastCallUnwind => Some("__cbindgen_abi_fastcall"),
            FunctionAbi::ThisCallUnwind => Some("__cbindgen_abi_thiscall"),
        }
    }

    pub fn as_clike_definition(&self) -> Option<&'static str> {
        match *self {
            FunctionAbi::None | FunctionAbi::C | FunctionAbi::CUnwind => None,
            FunctionAbi::CDecl => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::CDeclUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::StdCall => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::StdCallUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::Win64 => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::Win64Unwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::SystemV64 => Some(indoc! {r#"
                // Compiler-specific sysv64 calling convention definition
                #if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
                // Clang: https://clang.llvm.org/docs/AttributeReference.html#sysv-abi
                #define __cbindgen_abi_sysv64 __attribute__((sysv_abi))
                #elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
                // ICX: See Clang
                #define __cbindgen_abi_sysv64 __attribute__((sysv_abi))
                #elif defined(__GNUC__) || defined(__GNUG__)
                // GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-ms_005fabi-function-attribute_002c-x86
                #define __cbindgen_abi_sysv64 __attribute__((sysv_abi))
                #elif defined(_MSC_VER)
                // MSVC: SystemV ABI is not available on MSVC, so we generate an error if it is used
                // as this will result in code that compiles, but may break at runtime
                #pragma message ( "The SystemV ABI is not available in MSVC but has been requested. This may result in code which breaks at runtime." )
                #define __cbindgen_abi_sysv64
                #else
                #pragma message ( "An unsupported compiler is in use. Functions declared as extern \"sysv64\" may break at runtime." )
                #define __cbindgen_abi_sysv64
                #endif
            "#}),
            FunctionAbi::SystemV64Unwind => Some(indoc! {r#"
                // Compiler-specific sysv64 calling convention definition
                #if defined(__clang__) && !defined(__INTEL_LLVM_COMPILER)
                // Clang: https://clang.llvm.org/docs/AttributeReference.html#sysv-abi
                #define __cbindgen_abi_sysv64_unwind __attribute__((sysv_abi))
                #elif defined(__clang__) && defined(__INTEL_LLVM_COMPILER)
                // ICX: See Clang
                #define __cbindgen_abi_sysv64_unwind __attribute__((sysv_abi))
                #elif defined(__GNUC__) || defined(__GNUG__)
                // GCC: https://gcc.gnu.org/onlinedocs/gcc/x86-Function-Attributes.html#index-ms_005fabi-function-attribute_002c-x86
                #define __cbindgen_abi_sysv64_unwind __attribute__((sysv_abi))
                #elif defined(_MSC_VER)
                // MSVC: SystemV ABI is not available on MSVC, so we generate an error if it is used
                // as this will result in code that compiles, but may break at runtime
                #pragma message ( "The SystemV ABI is not available in MSVC but has been requested. This may result in code which breaks at runtime." )
                #define __cbindgen_abi_sysv64_unwind
                #else
                #pragma message ( "An unsupported compiler is in use. Functions declared as extern \"sysv64\" may break at runtime." )
                #define __cbindgen_abi_sysv64_unwind
                #endif
            "#}),
            FunctionAbi::System => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::SystemUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::AApcs => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::AApcsUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::FastCall => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::FastCallUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::ThisCall => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::ThisCallUnwind => Some(indoc! {r#"
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
            "#}),
            FunctionAbi::EfiApi => Some(indoc! {r#"
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
            "#}),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Option<String>,
    pub ty: Type,
    pub array_length: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub path: Path,
    /// Path to the self-type of the function
    /// If the function is a method, this will contain the path of the type in the impl block
    pub self_type_path: Option<Path>,
    pub ret: Type,
    pub args: Vec<FunctionArgument>,
    /// Whether the declaration needs an "extern" keyword
    pub extern_decl: bool,
    /// The ABI of the declaration
    pub abi: FunctionAbi,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
    pub never_return: bool,
}

impl Function {
    pub fn load(
        path: Path,
        self_type_path: Option<&Path>,
        sig: &syn::Signature,
        extern_decl: bool,
        abi: FunctionAbi,
        attrs: &[syn::Attribute],
        mod_cfg: Option<&Cfg>,
    ) -> Result<Function, String> {
        let mut args = sig.inputs.iter().try_skip_map(|x| x.as_argument())?;
        if sig.variadic.is_some() {
            args.push(FunctionArgument {
                name: None,
                ty: Type::Primitive(super::PrimitiveType::VaList),
                array_length: None,
            })
        }

        let (mut ret, never_return) = Type::load_from_output(&sig.output)?;

        if let Some(self_path) = self_type_path {
            for arg in &mut args {
                arg.ty.replace_self_with(self_path);
            }
            ret.replace_self_with(self_path);
        }

        Ok(Function {
            path,
            self_type_path: self_type_path.cloned(),
            ret,
            args,
            extern_decl,
            abi,
            cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
            annotations: AnnotationSet::load(attrs)?,
            documentation: Documentation::load(attrs),
            never_return,
        })
    }

    pub fn swift_name(&self, config: &Config) -> Option<String> {
        if config.language == Language::Cython {
            return None;
        }
        // If the symbol name starts with the type name, separate the two components with '.'
        // so that Swift recognises the association between the method and the type
        let (ref type_prefix, ref type_name) = match self.self_type_path {
            Some(ref type_name) => {
                let type_name = type_name.to_string();
                if !self.path.name().starts_with(&type_name) {
                    return Some(self.path.to_string());
                }
                (format!("{}.", type_name), type_name)
            }
            None => ("".to_string(), "".to_string()),
        };

        let item_name = self
            .path
            .name()
            .trim_start_matches(type_name)
            .trim_start_matches('_');

        let item_args = {
            let mut items = Vec::with_capacity(self.args.len());
            for arg in self.args.iter() {
                items.push(format!("{}:", arg.name.as_ref()?.as_str()));
            }
            items.join("")
        };
        Some(format!("{}{}({})", type_prefix, item_name, item_args))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn simplify_standard_types(&mut self, config: &Config) {
        self.ret.simplify_standard_types(config);
        for arg in &mut self.args {
            arg.ty.simplify_standard_types(config);
        }
    }

    pub fn add_dependencies(&self, library: &Library, out: &mut Dependencies) {
        self.ret.add_dependencies(library, out);
        for arg in &self.args {
            arg.ty.add_dependencies(library, out);
        }
    }

    pub fn add_monomorphs(&self, library: &Library, out: &mut Monomorphs) {
        self.ret.add_monomorphs(library, out);
        for arg in &self.args {
            arg.ty.add_monomorphs(library, out);
        }
    }

    pub fn mangle_paths(&mut self, monomorphs: &Monomorphs) {
        self.ret.mangle_paths(monomorphs);
        for arg in &mut self.args {
            arg.ty.mangle_paths(monomorphs);
        }
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ret.resolve_declaration_types(resolver);
        for arg in &mut self.args {
            arg.ty.resolve_declaration_types(resolver);
        }
    }

    pub fn rename_for_config(&mut self, config: &Config) {
        // Rename the types used in arguments
        let generic_params = Default::default();
        self.ret.rename_for_config(config, &generic_params);

        // Apply rename rules to argument names
        let rules = self
            .annotations
            .parse_atom::<RenameRule>("rename-all")
            .unwrap_or(config.function.rename_args);

        if let Some(r) = rules.not_none() {
            let args = std::mem::take(&mut self.args);
            self.args = args
                .into_iter()
                .map(|arg| {
                    let name = arg
                        .name
                        .map(|n| r.apply(&n, IdentifierType::FunctionArg).into_owned());
                    FunctionArgument {
                        name,
                        ty: arg.ty,
                        array_length: None,
                    }
                })
                .collect()
        }

        // Escape C/C++ reserved keywords used in argument names, and
        // recursively rename argument types.
        for arg in &mut self.args {
            arg.ty.rename_for_config(config, &generic_params);
            if let Some(ref mut name) = arg.name {
                reserved::escape(name);
            }
        }

        // Save the array length of the pointer arguments which need to use
        // the C-array notation
        if let Some(tuples) = self.annotations.list("ptrs-as-arrays") {
            let mut ptrs_as_arrays: HashMap<String, String> = HashMap::new();
            for str_tuple in tuples {
                let parts: Vec<&str> = str_tuple[1..str_tuple.len() - 1]
                    .split(';')
                    .map(|x| x.trim())
                    .collect();
                if parts.len() != 2 {
                    warn!(
                        "{:?} does not follow the correct syntax, so the annotation is being ignored",
                        parts
                    );
                    continue;
                }
                ptrs_as_arrays.insert(parts[0].to_string(), parts[1].to_string());
            }

            for arg in &mut self.args {
                match arg.ty {
                    Type::Ptr { .. } => {}
                    _ => continue,
                }
                let name = match arg.name {
                    Some(ref name) => name,
                    None => continue,
                };
                arg.array_length = ptrs_as_arrays.get(name).cloned();
            }
        }
    }
}

trait SynFnArgHelpers {
    fn as_argument(&self) -> Result<Option<FunctionArgument>, String>;
}

fn gen_self_type(receiver: &syn::Receiver) -> Result<Type, String> {
    let mut self_ty = Type::Path(GenericPath::self_path());

    // Custom self type
    if receiver.colon_token.is_some() {
        self_ty = Type::load(receiver.ty.as_ref())?.unwrap_or(self_ty);
    }

    if receiver.reference.is_none() {
        return Ok(self_ty);
    }

    let is_const = receiver.mutability.is_none();
    Ok(Type::Ptr {
        ty: Box::new(self_ty),
        is_const,
        is_nullable: false,
        is_ref: false,
    })
}

impl SynFnArgHelpers for syn::FnArg {
    fn as_argument(&self) -> Result<Option<FunctionArgument>, String> {
        match *self {
            syn::FnArg::Typed(syn::PatType {
                ref pat, ref ty, ..
            }) => {
                let ty = match Type::load(ty)? {
                    Some(x) => x,
                    None => return Ok(None),
                };
                let name = match **pat {
                    syn::Pat::Wild(..) => None,
                    syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => {
                        if ty == Type::Primitive(super::PrimitiveType::VaList) {
                            None
                        } else {
                            Some(ident.unraw().to_string())
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Parameter has an unsupported argument name: {:?}",
                            pat
                        ))
                    }
                };
                if let Type::Array(..) = ty {
                    return Err("Array as function arguments are not supported".to_owned());
                }
                Ok(Some(FunctionArgument {
                    name,
                    ty,
                    array_length: None,
                }))
            }
            syn::FnArg::Receiver(ref receiver) => Ok(Some(FunctionArgument {
                name: Some("self".to_string()),
                ty: gen_self_type(receiver)?,
                array_length: None,
            })),
        }
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Procedural macro attributes for cbindgen.
//!
//! This crate provides the `#[cbindgen::namespace]` attribute that can be used
//! to specify C++ namespaces for individual functions in generated headers.
//!
//! # Example
//!
//! ```rust,ignore
//! use cbindgen_macro::namespace;
//!
//! #[namespace = "ffi::bar"]
//! #[no_mangle]
//! pub extern "C" fn foo() {}
//! ```
//!
//! The attribute itself is a no-op at compile time - it simply passes through
//! the item unchanged. However, cbindgen parses this attribute from the source
//! code to determine the C++ namespace for the function in the generated header.

use proc_macro::TokenStream;

/// Specifies a C++ namespace for a function in cbindgen-generated headers.
///
/// This attribute is a no-op at compile time but is parsed by cbindgen to
/// determine where to place the function declaration in the generated C++ header.
///
/// # Example
///
/// ```rust,ignore
/// #[cbindgen_macro::namespace = "ffi::bar"]
/// #[no_mangle]
/// pub extern "C" fn foo() {}
/// ```
///
/// This will generate the following C++ code:
///
/// ```cpp
/// extern "C" {
///
/// namespace ffi {
/// namespace bar {
///
/// void foo();
///
/// }  // namespace bar
/// }  // namespace ffi
///
/// }  // extern "C"
/// ```
#[proc_macro_attribute]
pub fn namespace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This is a no-op - we just pass through the item unchanged.
    // cbindgen parses the attribute directly from the source code.
    item
}

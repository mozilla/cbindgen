/// Matches glob patterns such as `src/*/mod.rs`.
///
/// A C comment like /* this */ in the docs must not end the generated comment,
/// and neither must `**/*.rs`.
#[no_mangle]
pub extern "C" fn root() {}

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// Matches glob patterns such as `src/*/mod.rs`.
///
/// A C comment like /* this */ in the docs must not end the generated comment,
/// and neither must `**/*.rs`.
void root();

}  // extern "C"

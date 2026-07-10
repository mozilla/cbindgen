#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint8_t BYTES[3] = { 97, 98, 99, };

constexpr static const uint8_t SLICE[19] = { 110, 111, 116, 32, 110, 117, 108, 108, 0, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, };

constexpr static const uint8_t BINARY[4] = { 0, 255, 16, 127, };

constexpr static const uint8_t EMPTY[0] = { };

/// A documented byte-string constant.
constexpr static const uint8_t DOCUMENTED[3] = { 100, 111, 99, };

extern "C" {

void root();

}  // extern "C"

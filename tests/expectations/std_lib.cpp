#include <cstdarg>
#include <cstdint>
#include <cstdlib>

template<typename T>
struct Option;

template<typename T, typename E>
struct Result;

struct String;

template<typename T>
struct Vec;

extern "C" {

void root(const Vec<String> *a, const Option<int32_t> *b, const Result<int32_t, String> *c);

} // extern "C"

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint8_t SMALL # = 255

  const uint64_t LARGE # = 18446744073709551615ull

  const int32_t NEGATIVE # = -42

  const uint32_t MASK # = ~0u

  const uintptr_t ARITHMETIC # = ((4 + 2) * 3)

  const uint32_t SHIFT # = (1u << 31)

  const uint32_t BITWISE # = (5 | 2)

  const bool ENABLED # = True

  const uint32_t CHARACTER # = 'x'

  const float FLOAT # = 0.1

  const double DOUBLE # = 0.3333333333333333

  const double FLOAT_DIVISION # = (1.0 / 3.0)

  const int64_t NESTED_NEGATION # = -~42

  const uint32_t BOUND # = UINT32_MAX

  const int32_t SIGNED_MIN # = INT32_MIN

  const uint8_t *POINTER # = <const uint8_t*>1

  cdef struct Aggregate:
    uint32_t value;
  const uint32_t Aggregate_ASSOCIATED # = 6

  ctypedef uint32_t Alias;

  const uint8_t ARRAY[2] # = [ 1, 2, ]

  const char STRING[] # = "text"

  const Aggregate AGGREGATE # = <Aggregate>{ 4 }

  const Alias ALIAS # = 5

  void use_types(Aggregate, Alias);

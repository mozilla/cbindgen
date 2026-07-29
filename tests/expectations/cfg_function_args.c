#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void cfg_function_args(
                       uint32_t first
#if defined(DEFINED)
                       , uint32_t enabled
#endif
#if defined(UNDEFINED)
                       , uint32_t disabled
#endif
                       , uint32_t last
);

void cfg_first_argument(
#if defined(DEFINED)
                        uint32_t enabled
#endif
#if (defined(DEFINED))
                        ,
#endif
                        uint32_t last
);

void cfg_all_arguments(
#if !(defined(DEFINED) || defined(UNDEFINED))
                       void
#endif
#if defined(DEFINED)
                       uint32_t enabled
#endif
#if defined(UNDEFINED)
#if (defined(DEFINED))
                       ,
#endif
                       uint32_t disabled
#endif
);

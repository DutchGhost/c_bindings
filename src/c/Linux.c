#include <stdint.h>

uint64_t c_clzl(uint64_t x) {
    return __builtin_clzl(x);
}
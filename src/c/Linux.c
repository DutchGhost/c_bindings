#include <stdint.h>

uint64_t clzl(uint64_t x) {
    return __builtin_clzl(x);
}
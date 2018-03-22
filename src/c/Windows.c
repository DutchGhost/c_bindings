#include <stdint.h>

uint64_t __inline __builtin_clz(uint64_t x) {
    unsigned long r = 0;
    _BitScanReverse(&r, x);
    return (31-r);
}

uint64_t c_clzl(uint64_t x)
{
    uint64_t u32 = (x >> 32);
    uint64_t result = u32 ? __builtin_clz(u32) : 32;
    if (result == 32) {
        u32 = x & 0xFFFFFFFFUL;
        result += (u32 ? __builtin_clz(u32) : 32);
    }
    return result;
}
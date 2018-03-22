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

uint64_t c_atoi(const char * b, const char * e) {
    static const uint64_t pow10[20] = {
        1000000000000000000UL,
        100000000000000000UL,
        10000000000000000UL,
        1000000000000000UL,
        100000000000000UL,
        10000000000000UL,
        1000000000000UL,
        100000000000UL,
        10000000000UL,
        1000000000UL,
        100000000UL,
        10000000UL,
        1000000UL,
        100000UL,
        10000UL,
        10000UL,
        1000UL,
        100UL,
        10UL,
        1UL,
    };

    uint64_t result = 0;

    uint64_t i = 20 - (e - b);

    for (; b != e; ++b) {
        uint64_t d = (*b) - '0';

        if (d > 10) {
            return 0;
        }
        result += pow10[i++] * d;
    }
    return result;
}
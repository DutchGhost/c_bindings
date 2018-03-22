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

uint64_t c_atoi(const char * b, uint64_t length) {
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

    uint64_t i = 20 - length;

    for(; length >= 4; b += 4, length -= 4) {
        uint64_t d1 = b[0] - '0';
        // if (d1 > 9) {
        //     return 0;
        // }
        uint64_t r1 = pow10[i++] * d1;

        uint64_t d2 = b[1] - '0';
        // if (d2 > 9) {
        //     return 0;
        // }
        uint64_t r2 = pow10[i++] * d2;

        uint64_t d3 = b[2] - '0';
        // if (d3 > 9) {
        //     return 0;
        // }
        uint64_t r3 = pow10[i++] * d3;

        uint64_t d4 = b[3] - '0';
        // if (d4 > 9) {
        //     return 0;
        // }
        uint64_t r4 = pow10[i++] * d4;

        result += r1 + r2 + r3 + r4;
    }

    for (; length--; b++) {
        uint64_t d = b[0] - '0';

        // if (d > 9) {
        //     return 0;
        // }

        result += pow10[i++] * d;
    }
    return result;
}